## Muhammad Haekal Kalipaksi

## 2206817490

### Reflections

**How many data your publlsher program will send to the message broker in one run?**

```rs
let users: Vec<&str> = vec!["Amir", "Budi", "Cica", "Dira", "Emir"];

for (i, user) in users.iter().enumerate() {
    let user_id = (i + 1).to_string();
    let user_name = format!("2206817490-{}", user);
    let message: UserCreatedEventMesage = UserCreatedEventMesage { user_id, user_name };
    _ = publisher.send(String::from("user_created"), message)
}
```

Based on code above the publisher will send 5 user_created data with messages for user "Amir", "Budi", "Cica", "Dira", "Emir"

**The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?**

The publisher will send the messages into the same messages broker and the subcriber will receive the data.

**Running RabbitMq as message broker**

![init-raabit](images/init-rabbit.png?raw=true)

**Sending and processing event**

There are three events that run concurenntly:

**RabbitMq**

![rabbit-mq](images/rabbit-running.png)

**Subscriber**

![subscriber-running](images/subscriber-running.png)

**Publisher**

![publisher-running](images/publisher-running.png)

What happen is rabbit mq is running, and subscriber will listen for messages from rabbitmq, when i run publisher it will send messages to the rabbitmq, finally subscriber will receive the messages.

**Monitoring chart based on publisher**

![monitor](images/spike.png)

Spike is representing that rabbitmq or messages broker get the messages and send it to subscriber.
