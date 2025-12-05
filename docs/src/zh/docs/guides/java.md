# 在 Java 中使用 Rudis

本指南介绍了如何在 Java 应用程序中使用 Rudis。

## 安装

### Maven

将以下依赖项添加到您的 `pom.xml` 文件中：

```xml
<dependency>
    <groupId>org.rudis</groupId>
    <artifactId>rudis-java-client</artifactId>
    <version>0.1.0</version>
</dependency>
```

### Gradle

将以下内容添加到您的 `build.gradle` 文件中：

```gradle
implementation 'org.rudis:rudis-java-client:0.1.0'
```

## 基本用法

以下是一个简单的示例，展示如何连接到 Rudis 并执行操作：

```java
import org.rudis.client.RudisClient;

public class RudisExample {
    public static void main(String[] args) {
        try {
            RudisClient client = new RudisClient("127.0.0.1", 6379);
            
            // 设置一个键
            client.set("key", "value");
            
            // 获取一个键
            String value = client.get("key");
            System.out.println("值: " + value);
            
            client.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

## 连接管理

对于生产应用程序，请使用连接池：

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

## 错误处理

Rudis 提供了健壮的错误处理机制：

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
                System.err.println("设置键失败: " + e.getMessage());
                // 处理特定的 Rudis 异常
            }
            
            client.close();
        } catch (Exception e) {
            System.err.println("连接失败: " + e.getMessage());
        }
    }
}
```