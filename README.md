### Tutorial 8

#### Bramantyo Priyo Utomo - 2206821563

1. The publisher program sends five messages to the message broker in one run. Each call to publish_event() sends one message to the "user_created" queue.
2. When both the publisher and subscriber programs use the same URL amqp://guest:guest@localhost:5672, it means they are connecting to the same AMQP broker instance using the same credentials and connection parameters. This is why the subscriber program can receive the messages sent by the publisher program.

### Sending and Processing Events

1. The publisher program sends five messages to the message broker in one run. Each call to publish_event() sends one message to the "user_created" queue.

- Subscriber
  ![img1](assets/iamges/Subscriber.png)
- Publisher
  ![img2](assets/iamges/Publisher.png)
- Docker
  ![img3](assets/iamges/Docker.png)

### Monitoring Chart Based on Publisher

![img3](assets/iamges/RabbitMQ.png)

We see from the chart that the publisher program sends five messages to the message broker in one run.
