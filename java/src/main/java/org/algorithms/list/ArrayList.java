package org.algorithms.list;

import java.util.function.Consumer;
import java.util.Optional;

public class ArrayList<T> {
  private T[] values;
  private int size;
  private int allocated;

  @SuppressWarnings("unchecked")
  public ArrayList(int initialAllocated) {
    this.size = 0;
    this.allocated = initialAllocated;
    this.values = (T[]) new Object[this.allocated];
  }

  public ArrayList() {
    this(4);
  }

  @SuppressWarnings("unchecked")
  public void addLast(T value) {
    if (this.size + 1 > this.allocated) {
      int newAllocated = this.allocated << 1;
      T[] newArray = (T[]) new Object[newAllocated];
      for (int idx = 0; idx < this.size; idx++) {
        newArray[idx] = this.values[idx];
      }
      this.allocated = newAllocated;
      this.values = newArray;
    }
    this.values[this.size] = value;
    this.size += 1;
  }

  @SuppressWarnings("unchecked")
  public void removeLast() {
    if (this.size > 0) {
      int newSize = this.size - 1;
      int newAllocated = this.allocated >> 1;
      if (newSize < newAllocated) {
        T[] newArray = (T[]) new Object[newAllocated];
        for (int idx = 0; idx < newSize; idx++) {
          newArray[idx] = this.values[idx];
        }
      }
      this.size = newSize;
      this.allocated = newAllocated;
    }
  }

  public Optional<T> get(int index) {
    if (index > this.size - 1) {
      return Optional.empty();
    }
    return Optional.of(this.values[index]);
  }

  public void forEach(Consumer<T> consumer) {
    for (int idx = 0; idx < this.size; idx++) {
      consumer.accept(this.values[idx]);
    }
  }
}