# Using Rudis in Java

This guide explains how to use Rudis in Java applications.

## Installation

### Maven

Add the following dependency to your `pom.xml` file:

```xml
<dependency>
    <groupId>org.rudis</groupId>
    <artifactId>rudis-java-client</artifactId>
    <version>0.1.0</version>
</dependency>
```

### Gradle

Add the following to your `build.gradle` file:

```gradle
implementation 'org.rudis:rudis-java-client:0.1.0'
```

## Basic Usage

Here's a simple example of how to connect to Rudis and perform operations:

```java
import org.rudis.client.RudisClient;

public class RudisExample {
    public static void main(String[] args) {
        try {
            RudisClient client = new RudisClient("127.0.0.1", 6379);
            
            // Set a key
            client.set("key", "value");
            
            // Get a key
            String value = client.get("key");
            System.out.println("Value: " + value);
            
            client.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

## Connection Management

For production applications, use connection pooling:

```java
import org.rudis.client.RudisClient;
import org.rudis.client.RudisPool;

public class RudisPoolExample {
    public static void main(String[] args) {
        try {
            RudisPool pool = new RudisPool("127.0.0.1", 6379, 10);
            
            RudisClient client = pool.getResource();
            client.set("key", "value");
            
            pool.returnResource(client);
            pool.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

## Error Handling

Rudis provides robust error handling mechanisms:

```java
import org.rudis.client.RudisClient;
import org.rudis.client.RudisException;

public class RudisErrorHandlingExample {
    public static void main(String[] args) {
        try {
            RudisClient client = new RudisClient("127.0.0.1", 6379);
            
            try {
                client.set("key", "value");
            } catch (RudisException e) {
                System.err.println("Failed to set key: " + e.getMessage());
                // Handle specific Rudis exceptions
            }
            
            client.close();
        } catch (Exception e) {
            System.err.println("Failed to connect: " + e.getMessage());
        }
    }
}
```