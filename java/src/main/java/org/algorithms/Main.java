package org.algorithms;

import org.algorithms.list.ArrayList;

public class Main {
  public static void main(String[] args) {
    ArrayList<Number> list = new ArrayList<>(2);
    list.addLast(1);
    list.addLast(2);
    list.addLast(3.2f);
    list.forEach(e -> System.out.println(e));
    list.removeLast();
    list.removeLast();
    System.out.println("After remove");
    list.forEach(e -> System.out.println(e));
  }
}