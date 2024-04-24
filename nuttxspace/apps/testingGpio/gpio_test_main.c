/****************************************************************************
 * apps/examples/hello/hello_main.c
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
#include <stdio.h>
#include <arch/board/board.h>

// To run make menuconfig -> drivers -> gpio support -> gpio drivers
#if defined(CONFIG_DEV_GPIO) && !defined(CONFIG_GPIO_LOWER_HALF)

/****************************************************************************
 * Private Types
 ****************************************************************************/
struct gpio_dev_s gpio;
struct stm32gpio_dev_s
{
  struct gpio_dev_s gpio;
  uint8_t id;
};

struct stm32gpint_dev_s
{
  struct stm32gpio_dev_s stm32gpio;
  pin_interrupt_t callback;
};

/****************************************************************************
 * Private Function Prototypes
 ****************************************************************************/

static int gpin_read(struct gpio_dev_s *dev, bool *value);
static int gpout_read(struct gpio_dev_s *dev, bool *value);
static int gpout_write(struct gpio_dev_s *dev, bool value);
static int gpint_read(struct gpio_dev_s *dev, bool *value);
static int gpint_attach(struct gpio_dev_s *dev,
                        pin_interrupt_t callback);
static int gpint_enable(struct gpio_dev_s *dev, bool enable);

/****************************************************************************
 * Private Data
 ****************************************************************************/

static const struct gpio_operations_s gpin_ops =
{
  .go_read   = gpin_read,
  .go_write  = NULL,
  .go_attach = NULL,
  .go_enable = NULL,
};

static const struct gpio_operations_s gpout_ops =
{
  .go_read   = gpout_read,
  .go_write  = gpout_write,
  .go_attach = NULL,
  .go_enable = NULL,
};

static const struct gpio_operations_s gpint_ops =
{
  .go_read   = gpint_read,
  .go_write  = NULL,
  .go_attach = gpint_attach,
  .go_enable = gpint_enable,
};

#if BOARD_NGPIOIN > 0
/* This array maps the GPIO pins used as INPUT */

static const uint32_t g_gpioinputs[BOARD_NGPIOIN] =
{
  GPIO_IN1,
  GPIO_IN2,
  GPIO_IN3,
  GPIO_IN4,
};

static struct stm32gpio_dev_s g_gpin[BOARD_NGPIOIN];
#endif

#if BOARD_NGPIOOUT
/* This array maps the GPIO pins used as OUTPUT */

static const uint32_t g_gpiooutputs[BOARD_NGPIOOUT] =
{
  GPIO_LD1,
  GPIO_LD2,
  GPIO_LD3,
  GPIO_OUT1,
  GPIO_OUT2,
  GPIO_OUT3,
  GPIO_OUT4,
  GPIO_OUT5,
#if !defined(CONFIG_STM32F7_TIM1_CH1NOUT)
  GPIO_OUT6,
#endif
#if !defined(CONFIG_STM32F7_TIM1_CH2NOUT)  
  GPIO_OUT7,
#endif
};

static struct stm32gpio_dev_s g_gpout[BOARD_NGPIOOUT];
#endif

#if BOARD_NGPIOINT > 0
/* This array maps the GPIO pins used as INTERRUPT INPUTS */

static const uint32_t g_gpiointinputs[BOARD_NGPIOINT] =
{
  GPIO_INT1,
};

static struct stm32gpint_dev_s g_gpint[BOARD_NGPIOINT];
#endif
/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * stub_main
 ****************************************************************************/
#endif
/* ADC1 */

#define GPIO_ADC1_IN0   GPIO_ADC1_IN0_0   /* PA0 */
#define GPIO_ADC1_IN1   GPIO_ADC1_IN1_0   /* PA1 */
#define GPIO_ADC1_IN2   GPIO_ADC1_IN2_0   /* PA2 */
#define GPIO_ADC1_IN3   GPIO_ADC1_IN3_0   /* PA3 */
#define GPIO_ADC1_IN4   GPIO_ADC1_IN4_0   /* PA4 */
#define GPIO_ADC1_IN5   GPIO_ADC1_IN5_0   /* PA5 */
#define GPIO_ADC1_IN6   GPIO_ADC1_IN6_0   /* PA6 */
#define GPIO_ADC1_IN7   GPIO_ADC1_IN7_0   /* PA7 */
#define GPIO_ADC1_IN8   GPIO_ADC1_IN8_0   /* PB0 */
#define GPIO_ADC1_IN9   GPIO_ADC1_IN9_0   /* PB1 */
#define GPIO_ADC1_IN10  GPIO_ADC1_IN10_0  /* PC0 */
#define GPIO_ADC1_IN11  GPIO_ADC1_IN11_0  /* PC1 */
#define GPIO_ADC1_IN12  GPIO_ADC1_IN12_0  /* PC2 */
#define GPIO_ADC1_IN13  GPIO_ADC1_IN13_0  /* PC3 */
#define GPIO_ADC1_IN14  GPIO_ADC1_IN14_0  /* PC4 */
#define GPIO_ADC1_IN15  GPIO_ADC1_IN15_0  /* PC5 */

void pin_read_test(struct gpio_dev_s *dev)
{

}

int main(int argc, FAR char *argv[])
{
  //  *  PA5   SPI1_SCK  CN12-11
  // GPIO_ADC1_IN5_0
  // tehre are outpins and in pins, find them and define them, first
  struct gpio_dev_s gpio;
  gpio.gp_pintype = GPIO_ADC1_IN5_0;
  gpio.gpout_ops = gpout_ops;
  printf(" initializing pins \n");
  stm32_gpio_initialize();
  
  printf(" writing high to pin \n");
  gpout_write(gpio, 1);
  printf(" sleeping for 3 \n");
  sleep(3);


  printf(" reading high from pin \n");
  gpout_read(gpio, 1);
  printf(" sleeping for 3 \n");
  sleep(3);


// interupt method

  return 0;
}
