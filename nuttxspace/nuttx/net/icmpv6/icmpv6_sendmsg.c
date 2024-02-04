/****************************************************************************
 * net/icmpv6/icmpv6_sendmsg.c
 *
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.  The
 * ASF licenses this file to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance with the
 * License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
 * License for the specific language governing permissions and limitations
 * under the License.
 *
 ****************************************************************************/

/****************************************************************************
 * Included Files
 ****************************************************************************/

#include <nuttx/config.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <assert.h>
#include <debug.h>

#include <netinet/in.h>
#include <net/if.h>
#include <arpa/inet.h>

#include <nuttx/semaphore.h>
#include <nuttx/mm/iob.h>
#include <nuttx/net/netconfig.h>
#include <nuttx/net/net.h>
#include <nuttx/net/netdev.h>
#include <nuttx/net/netstats.h>
#include <nuttx/net/ip.h>
#include <nuttx/net/icmpv6.h>

#include "utils/utils.h"
#include "socket/socket.h"
#include "netdev/netdev.h"
#include "devif/devif.h"
#include "inet/inet.h"
#include "icmpv6/icmpv6.h"

#ifdef CONFIG_NET_ICMPv6_SOCKET

/****************************************************************************
 * Private Types
 ****************************************************************************/

struct icmpv6_sendto_s
{
  FAR struct devif_callback_s *snd_cb; /* Reference to callback instance */
  sem_t snd_sem;                       /* Use to manage the wait for send
                                        * complete */
  struct in6_addr snd_toaddr;          /* The peer to send the request to */
  FAR const uint8_t *snd_buf;          /* ICMPv6 header + data payload */
  uint16_t snd_buflen;                 /* Size of the ICMPv6 header + data
                                        * payload */
  int16_t snd_result;                  /* 0: success; <0:negated errno on
                                        * fail */
};

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/****************************************************************************
 * Name: sendto_request
 *
 * Description:
 *   Setup to send an ICMPv6 request packet
 *
 * Input Parameters:
 *   dev    - The device driver structure to use in the send operation
 *   pstate - Reference to an instance of the ICMPv6 sendto state structure
 *
 * Returned Value:
 *   None
 *
 * Assumptions:
 *   The network is locked.
 *
 ****************************************************************************/

static void sendto_request(FAR struct net_driver_s *dev,
                           FAR struct icmpv6_sendto_s *pstate)
{
  FAR struct icmpv6_echo_request_s *icmpv6;

  /* Set-up to send that amount of data. */

  devif_send(dev, pstate->snd_buf, pstate->snd_buflen, IPv6_HDRLEN);
  if (dev->d_sndlen != pstate->snd_buflen)
    {
      return;
    }

  IFF_SET_IPv6(dev->d_flags);

  /* The total length to send is the size of the application data plus the
   * IP and ICMPv6 headers (and, eventually, the Ethernet header)
   */

  dev->d_len = IPv6_HDRLEN + pstate->snd_buflen;

  ipv6_build_header(IPv6BUF, pstate->snd_buflen, IP_PROTO_ICMP6,
                    netdev_ipv6_srcaddr(dev, pstate->snd_toaddr.s6_addr16),
                    pstate->snd_toaddr.s6_addr16, 255, 0);

  /* Copy the ICMPv6 request and payload into place after the IPv6 header */

  icmpv6 = IPBUF(IPv6_HDRLEN);

  /* Calculate the ICMPv6 checksum over the ICMPv6 header and payload. */

  icmpv6->chksum = 0;
  icmpv6->chksum = ~icmpv6_chksum(dev, IPv6_HDRLEN);
  if (icmpv6->chksum == 0)
    {
      icmpv6->chksum = 0xffff;
    }

  ninfo("Outgoing ICMPv6 packet length: %d\n", dev->d_len);

#ifdef CONFIG_NET_STATISTICS
  g_netstats.icmpv6.sent++;
  g_netstats.ipv6.sent++;
#endif
}

/****************************************************************************
 * Name: sendto_eventhandler
 *
 * Description:
 *   This function is called with the network locked to perform the actual
 *   ECHO request and/or ECHO reply actions when polled by the lower, device
 *   interfacing layer.
 *
 * Input Parameters:
 *   dev        The structure of the network driver that generated the
 *              event
 *   pvpriv     An instance of struct icmpv6_sendto_s cast to (void *)
 *   flags      Set of events describing why the callback was invoked
 *
 * Returned Value:
 *   Modified value of the input flags
 *
 * Assumptions:
 *   The network is locked.
 *
 ****************************************************************************/

static uint16_t sendto_eventhandler(FAR struct net_driver_s *dev,
                                    FAR void *pvpriv, uint16_t flags)
{
  FAR struct icmpv6_sendto_s *pstate = pvpriv;

  ninfo("flags: %04x\n", flags);

  if (pstate != NULL)
    {
      /* Check if the network is still up */

      if ((flags & NETDEV_DOWN) != 0)
        {
          nerr("ERROR: Interface is down\n");
          pstate->snd_result = -ENETUNREACH;
          goto end_wait;
        }

      /* Check:
       *   If the outgoing packet is available (it may have been claimed
       *   by a sendto event handler serving a different thread)
       * -OR-
       *   If the output buffer currently contains unprocessed incoming
       *   data.
       * -OR-
       *   If we have already sent the ECHO request
       *
       * In the first two cases, we will just have to wait for the next
       * polling cycle.
       */

      if (dev->d_sndlen <= 0 &&           /* Packet available */
          (flags & ICMPv6_NEWDATA) == 0)  /* No incoming data */
        {
          /* Send the ICMPv6 echo request.  */

          ninfo("Send ICMPv6 ECHO request\n");

          sendto_request(dev, pstate);
          if (dev->d_sndlen > 0)
            {
              pstate->snd_result = OK;
              goto end_wait;
            }
        }

      /* Continue waiting */
    }

  return flags;

end_wait:
  ninfo("Resuming\n");

  /* Do not allow any further callbacks */

  pstate->snd_cb->flags   = 0;
  pstate->snd_cb->priv    = NULL;
  pstate->snd_cb->event   = NULL;

  /* Wake up the waiting thread */

  nxsem_post(&pstate->snd_sem);
  return flags;
}

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * Name: icmpv6_sendmsg
 *
 * Description:
 *   Implements the sendmsg() operation for the case of the IPPROTO_ICMP6
 *   socket.  The 'buf' parameter points to a block of memory that includes
 *   an ICMPv6 request header, followed by any payload that accompanies the
 *   request.  The 'len' parameter includes both the size of the ICMPv6
 *   header and the following payload.
 *
 * Input Parameters:
 *   psock    A pointer to a NuttX-specific, internal socket structure
 *   msg      Message to send
 *   flags    Send flags
 *
 * Returned Value:
 *   On success, returns the number of characters sent.  On error, a negated
 *   errno value is returned (see sendmsg() for the list of appropriate error
 *   values.
 *
 ****************************************************************************/

ssize_t icmpv6_sendmsg(FAR struct socket *psock, FAR struct msghdr *msg,
                       int flags)
{
  FAR const void *buf = msg->msg_iov->iov_base;
  size_t len = msg->msg_iov->iov_len;
  FAR const struct sockaddr *to = msg->msg_name;
  socklen_t tolen = msg->msg_namelen;
  FAR const struct sockaddr_in6 *inaddr;
  FAR struct net_driver_s *dev;
  FAR struct icmpv6_conn_s *conn;
  FAR struct icmpv6_echo_request_s *icmpv6;
  struct icmpv6_sendto_s state;
  ssize_t ret;

  /* Validity check, only single iov supported */

  if (msg->msg_iovlen != 1)
    {
      return -ENOTSUP;
    }

  if (to == NULL)
    {
      /* icmpv6_send() */

      /* ICMPv6 sockets cannot be bound and, hence, cannot support any
       * connection-oriented data transfer.
       */

      return -EDESTADDRREQ;
    }

  /* Some sanity checks */

  DEBUGASSERT(buf != NULL && to != NULL);

  if (len < ICMPv6_HDRLEN || tolen < sizeof(struct sockaddr_in6))
    {
      return -EINVAL;
    }

  conn   = psock->s_conn;
  inaddr = (FAR const struct sockaddr_in6 *)to;

  /* Get the device that will be used to route this ICMPv6 ECHO request */

#ifdef CONFIG_NET_BINDTODEVICE
  if (conn->sconn.s_boundto != 0)
    {
      dev = netdev_findbyindex(conn->sconn.s_boundto);
    }
  else
#endif
    {
      dev = netdev_findby_ripv6addr(g_ipv6_unspecaddr,
                                    inaddr->sin6_addr.s6_addr16);
    }

  if (dev == NULL)
    {
      nerr("ERROR: Not reachable\n");
      ret = -ENETUNREACH;
      goto errout;
    }

#ifndef CONFIG_NET_IPFRAG
  /* Sanity check if the request len is greater than the net payload len */

  if (len > NETDEV_PKTSIZE(dev) - (NET_LL_HDRLEN(dev) + IPv6_HDRLEN))
    {
      nerr("ERROR: Invalid packet length\n");
      return -EINVAL;
    }
#endif

  /* If we are no longer processing the same ping ID, then flush any pending
   * packets from the read-ahead buffer.
   *
   * REVISIT:  How to we free up any lingering responses if there are no
   * further pings?
   */

  icmpv6 = (FAR struct icmpv6_echo_request_s *)buf;
  if (psock->s_type != SOCK_RAW && (icmpv6->type != ICMPv6_ECHO_REQUEST ||
      icmpv6->id != conn->id || dev != conn->dev))
    {
      conn->id  = 0;
      conn->dev = NULL;

      iob_free_queue(&conn->readahead);
    }

#ifdef CONFIG_NET_ICMPv6_NEIGHBOR
  /* Make sure that the IP address mapping is in the Neighbor Table */

  ret = icmpv6_neighbor(dev, inaddr->sin6_addr.s6_addr16);
  if (ret < 0)
    {
      nerr("ERROR: Not reachable\n");
      ret = -ENETUNREACH;
      goto errout;
    }
#endif /* CONFIG_NET_ICMPv6_NEIGHBOR */

  /* Initialize the state structure */

  nxsem_init(&state.snd_sem, 0, 0);

  state.snd_result = -ENOMEM;           /* Assume allocation failure */
  state.snd_buf    = buf;               /* ICMPv6 header + data payload */
  state.snd_buflen = len;               /* Size of the ICMPv6 header+data payload */

  net_ipv6addr_copy(state.snd_toaddr.s6_addr16,
                    inaddr->sin6_addr.s6_addr16);

  net_lock();

  /* Set up the callback */

  state.snd_cb = icmpv6_callback_alloc(dev, conn);
  if (state.snd_cb)
    {
      state.snd_cb->flags   = (ICMPv6_POLL | NETDEV_DOWN);
      state.snd_cb->priv    = (FAR void *)&state;
      state.snd_cb->event   = sendto_eventhandler;

      /* Setup to receive ICMPv6 ECHO replies */

      if (psock->s_type != SOCK_RAW && icmpv6->type == ICMPv6_ECHO_REQUEST)
        {
          conn->id = icmpv6->id;
        }

      conn->dev = dev;

      /* Notify the device driver of the availability of TX data */

      netdev_txnotify_dev(dev);

      /* Wait for either the send to complete or for timeout to occur.
       * net_sem_timedwait will also terminate if a signal is received.
       */

      ret = net_sem_timedwait(&state.snd_sem,
                          _SO_TIMEOUT(conn->sconn.s_sndtimeo));
      if (ret < 0)
        {
          if (ret == -ETIMEDOUT)
            {
              /* Check if this device is on the same network as the
               * destination device.
               */

              if (!NETDEV_V6ADDR_ONLINK(dev, state.snd_toaddr.s6_addr16))
                {
                  /* Destination address was not on the local network served
                   * by this device.  If a timeout occurs, then the most
                   * likely reason is that the destination address is not
                   * reachable.
                   */

                  ret = -ENETUNREACH;
                }
              else
                {
                  ret = -EAGAIN;
                }
            }

          state.snd_result = ret;
        }

      icmpv6_callback_free(dev, conn, state.snd_cb);
    }

  nxsem_destroy(&state.snd_sem);

  net_unlock();

  /* Return the negated error number in the event of a failure, or the
   * number of bytes sent on success.
   */

  if (state.snd_result < 0)
    {
      nerr("ERROR: Return error=%d\n", state.snd_result);
      ret = state.snd_result;
      goto errout;
    }

  return len;

errout:
  conn->id  = 0;
  conn->dev = NULL;

  iob_free_queue(&conn->readahead);
  return ret;
}

#endif /* CONFIG_NET_ICMPv6_SOCKET */
