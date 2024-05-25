/*
 * Un commentaire.
 */
// Single line
/* Multi
 * Line
 */

/**
 * Documentation.
 *
 * @param  x  desc
 * @param  y  desc
 * @return    desc
 */

package com.module.module2.module3;

import lombok.Data;

@Data
public class UserDto {
    private String id;
}


import org.springframework.http.*;
import org.springframework.context.event.EventListener;

@Data
@Entity
@Table(name = "account")
public class AccountRecord 
{
    @Id
    @Column(name = "id")
    private long id;

    @Column(name = "first_name", nullable = false)
    private String firstName;

    @Column(name = "last_name", nullable = false)
    private String lastName;
}


public record AuthenticationRequest(
    String username,
    String password,
) 
{
}

public enum Gender 
{
    MALE,
    FEMALE,
}

@Component
public class EventQueue<T> 
{
    private final Queue<T> queue;

    public EventQueue()
    {
        this.queue = new ArrayDeque<>();
    }

    public void addEvent(T event)
    {
           queue.add(event);
    }

    public T next()
    {
        if (queue.isEmpty())
        {
            throw new NoSuchElementException();
        }

        return queue.remove();
    }
}

@Component
public class UseCaseEventHandler
{
    private final EventQueue<DomainApplicationEvent> queue;

    public UseCaseEventHandler(EventQueue<DomainApplicationEvent> eventEventQueue)
    {
        this.queue = eventEventQueue;
    }

    @EventListener
    public void handleEvent(DomainApplicationEvent event)
    {
        queue.addEvent(event);

        var eventFromQueue = queue.next();
        System.out.println(eventFromQueue.getId());
        System.out.println(eventFromQueue.getEventName());
        System.out.println(eventFromQueue.getLocalDateTime());
    }
}
