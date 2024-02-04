#!/usr/bin/env bash
# apps/tools/mkkconfig.sh
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.  The
# ASF licenses this file to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance with the
# License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
# WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
# License for the specific language governing permissions and limitations
# under the License.
#

# Get the input parameter list

USAGE="USAGE: mkkconfig.sh [-d] [-h] [-m <menu>] [-o <kconfig-file>]"
KCONFIG=Kconfig
unset MENU

while [ ! -z "$1" ]; do
  case $1 in
    -d )
      set -x
      ;;
    -m )
      shift
      MENU=$1
      ;;
    -o )
      shift
      KCONFIG=$1
      ;;
    -h )
      echo $USAGE
      exit 0
      ;;
    * )
      echo "ERROR: Unrecognized argument: $1"
      echo $USAGE
      exit 1
      ;;
    esac
  shift
done


if [ -f ${KCONFIG} ]; then
  rm ${KCONFIG} || { echo "ERROR: Failed to remove $PWD/${KCONFIG}"; exit 1; }
fi

echo mkkconfig in $PWD

KCONFIG_LIST=`ls -1 $PWD/*/Kconfig 2>/dev/null`

echo "#" > ${KCONFIG}
echo "# For a description of the syntax of this configuration file," >> ${KCONFIG}
echo "# see the file kconfig-language.txt in the NuttX tools repository." >> ${KCONFIG}
echo "#" >> ${KCONFIG}
echo "# This file is autogenerated, do not edit." >> ${KCONFIG}
echo "#" >> ${KCONFIG}
echo "" >> ${KCONFIG}

if [ ! -z "${MENU}" ]; then
  echo "menu \"${MENU}\"" >> ${KCONFIG}
fi

for FILE in ${KCONFIG_LIST}; do
  echo "source \"${FILE}\"" >> ${KCONFIG}
done

if [ ! -z "${MENU}" ]; then
  echo "endmenu # ${MENU}" >> ${KCONFIG}
fi
