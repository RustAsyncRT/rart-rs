/**
 * @file main.c
 * @author Matheus T. dos Santos (matheus.santos@edge.ufal.br)
 * @brief
 * @version 0.1
 * @date 03/04/2022
 *
 * @copyright Copyright (c) 2022
 *
 */
#include "rart-defines.h"
#include <zephyr.h>
#include <zbus.h>
#include "zbus_messages.h"

ZBUS_SUBSCRIBER_DECLARE(pong_subscriber, 4);
void pong_task()
{
    zbus_channel_index_t idx;
    while (!k_msgq_get(pong_subscriber.queue, &idx, K_FOREVER)) {
        // read
        struct ball_pos recv_msg = {0};
        ZBUS_CHAN_READ(pong, recv_msg, K_NO_WAIT);
        printk("[c]receive ball pos: <%d, %d>\n", recv_msg.x, recv_msg.y);

        // pub
        struct ball_pos send_msg = {.x = 7, .y = 12};
        ZBUS_CHAN_PUB(ping, send_msg, K_MSEC(1000));
    }
}
K_THREAD_DEFINE(pong_task_id, 512, pong_task, NULL, NULL, NULL, 3, 0, 0);

void main(void) {
    printk("[c]Starting RART loop...\n");
    main_task();
    printk("[c]End of RART loop\n");
}
