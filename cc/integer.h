#pragma once

class Integer {
public:
  Integer(int value) : _value(value) {}

  int get() const
  { return _value; }

  void set(int value)
  { _value = value; }

private:
  int _value;
};
