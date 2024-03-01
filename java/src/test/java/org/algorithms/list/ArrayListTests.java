package org.algorithms.list;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Optional;
import org.junit.jupiter.api.Test;

public class ArrayListTests {

  @Test
  public void shouldReturnAddedValue() {
    ArrayList<Integer> list = new ArrayList<>();

    list.addLast(5);

    assertEquals(list.get(0), Optional.of(5));
  }

  @Test
  public void shouldReturnEmptyWhenNoValueAdded() {
    ArrayList<Integer> list = new ArrayList<>();

    assertEquals(list.get(0), Optional.empty());
  }

  @Test
  public void shouldReturnEmptyWhenLastRemoved() {
    ArrayList<Integer> list = new ArrayList<>();

    list.addLast(5);
    list.removeLast();

    assertEquals(list.get(0), Optional.empty());
  }
}