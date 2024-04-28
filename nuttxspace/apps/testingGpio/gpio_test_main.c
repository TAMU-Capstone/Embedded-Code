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
#include <nuttx/compiler.h>
#include <stdint.h>
#include <stdio.h>
#include <arch/board/board.h>
#include <nuttx/clock.h>
#include <nuttx/wdog.h>
#include <nuttx/ioexpander/gpio.h>
#include <nuttx/irq.h>
#include "stm32_gpio.h"
#include <stdbool.h>
#include <assert.h>
#include <debug.h>
#include <nuttx/wdog.h>
#include <nuttx/compiler.h>

#include "chip.h"
#include "stm32_gpio.h"
// To run make menuconfig -> drivers -> gpio support -> gpio drivers
#if !defined(CONFIG_DEV_GPIO) && defined(CONFIG_GPIO_LOWER_HALF)
int main(int argc, FAR char *argv[])
{
  printf(" CONFIG_DEV_GPIO NOT DEFINED, LOOK AT DOCUMENTATION FOR PARAMS\n");
  return 0;
}
#endif

/****************************************************************************
 * Defined Types
 ****************************************************************************/

#if defined(CONFIG_DEV_GPIO) && !defined(CONFIG_GPIO_LOWER_HALF)
#define BOARD_NGPIOIN 4 /* Amount of GPIO Input pins */
#if defined(CONFIG_STM32F7_TIM1_CH1NOUT) && defined(CONFIG_STM32F7_TIM1_CH2NOUT)
#define BOARD_NGPIOOUT 8 /* Amount of GPIO Output pins */
#elif defined(CONFIG_STM32F7_TIM1_CH1NOUT) || defined(CONFIG_STM32F7_TIM1_CH2NOUT)
#define BOARD_NGPIOOUT 9 /* Amount of GPIO Output pins */
#else
#define BOARD_NGPIOOUT 10 /* Amount of GPIO Output pins */
#endif
#define BOARD_NGPIOINT 1 /* Amount of GPIO Input w/ Interruption pins */

#define GPIO_LD1 (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | GPIO_OUTPUT_CLEAR | \
                  GPIO_PORTB | GPIO_PIN0)
#define GPIO_LD2 (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | GPIO_OUTPUT_CLEAR | \
                  GPIO_PORTB | GPIO_PIN7)
#define GPIO_LD3 (GPIO_OUTPUT | GPIO_PUSHPULL | GPIO_SPEED_50MHz | GPIO_OUTPUT_CLEAR | \
                  GPIO_PORTB | GPIO_PIN14)

#define GPIO_LED_GREEN GPIO_LD1
#define GPIO_LED_BLUE GPIO_LD2
#define GPIO_LED_RED GPIO_LD3

// found in arch/arm/src/stm32f7/chip.h
#define GPIO_INT1 (GPIO_INPUT | GPIO_FLOAT | GPIO_PORTB | GPIO_PIN2)

#define GPIO_IN1 (GPIO_INPUT | GPIO_FLOAT | GPIO_PORTE | GPIO_PIN7)
#define GPIO_IN2 (GPIO_INPUT | GPIO_FLOAT | GPIO_PORTE | GPIO_PIN12)
#define GPIO_IN3 (GPIO_INPUT | GPIO_FLOAT | GPIO_PORTE | GPIO_PIN14)
#define GPIO_IN4 (GPIO_INPUT | GPIO_FLOAT | GPIO_PORTE | GPIO_PIN15)

#define GPIO_OUT1 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTE | GPIO_PIN4)
#define GPIO_OUT2 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTE | GPIO_PIN5)
#define GPIO_OUT3 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTE | GPIO_PIN6)
#define GPIO_OUT4 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTA | GPIO_PIN5)
#define GPIO_OUT5 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTF | GPIO_PIN12)
#if !defined(CONFIG_STM32F7_TIM1_CH1NOUT)
#define GPIO_OUT6 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTE | GPIO_PIN8)
#endif
#if !defined(CONFIG_STM32F7_TIM1_CH2NOUT)
#define GPIO_OUT7 (GPIO_OUTPUT | GPIO_SPEED_50MHz | \
                   GPIO_OUTPUT_SET | GPIO_PORTE | GPIO_PIN10)
#endif

// struct gpio_dev_s;
struct gpio_dev_s;
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
void stm32_gpio_initialize(void);

/****************************************************************************
 * Private Data
 ****************************************************************************/

static const struct gpio_operations_s gpin_ops =
    {
        .go_read = gpin_read,
        .go_write = NULL,
        .go_attach = NULL,
        .go_enable = NULL,
};

static const struct gpio_operations_s gpout_ops =
    {
        .go_read = gpout_read,
        .go_write = gpout_write,
        .go_attach = NULL,
        .go_enable = NULL,
};

static const struct gpio_operations_s gpint_ops =
    {
        .go_read = gpint_read,
        .go_write = NULL,
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

static const uint32_t cd[BOARD_NGPIOINT] =
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
/* ADC1 */

void pin_read_test(struct gpio_dev_s *dev)
{
}

int main(int argc, FAR char *argv[])
{
  //  *  PA5   SPI1_SCK  CN12-11
  // GPIO_ADC1_IN5_0
  // tehre are outpins and in pins, find them and define them, first  
  printf(" enter main \n");
  struct gpio_dev_s *gpio;
  gpio->gp_pintype = GPIO_OUT1;
  printf(" set pintype properly \n");
  gpio->gp_ops = &gpout_ops;
  printf(" set output operation properly \n");
  bool * readPin = 0;
  printf(" initializing pins \n");
  stm32_gpio_initialize();

  printf(" writing high to pin \n");
  gpout_write(gpio, readPin);
  printf(" sleeping for 3 \n");
  sleep(3);
  printf(" the pin value is %d", *readPin);

  printf(" reading high from pin \n");
  gpout_read(gpio, 1);
  printf(" sleeping for 3 \n");
  sleep(3);

  // interupt method

  return 0;
}

#endif