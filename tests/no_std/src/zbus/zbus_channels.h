ZBUS_CHANNEL(ping,
             false,
             false,
             struct ball_pos,
             NULL,
             ZBUS_OBSERVERS(ping_listener),
             ZBUS_MSG_INIT(.x = 0, .y = 0)
)

ZBUS_CHANNEL(pong,
             false,
             false,
             struct ball_pos,
             NULL,
             ZBUS_OBSERVERS(pong_subscriber),
             ZBUS_MSG_INIT(.x = 0, .y = 0)
)