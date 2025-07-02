# Generators
# are a way to create iterators in Python. They allow you to iterate over a sequence of values without storing the entire sequence in memory at once. This is particularly useful for large datasets or streams of data.
# A generator is defined using a function with the `yield` statement. When the function is called, it returns a generator object, which can be iterated over to get the values one at a time.
# Generators are often used in conjunction with the `yield from` statement, which allows you to iterate over an iterable returned by another function. they allow the pause and resumption of function execution, making them suitable for asynchronous programming and handling large datasets efficiently.

def odds(start, end):
    """Generator that yields odd numbers from start to end."""
    for odd_number in range(start, end + 1, 2):
        if odd_number % 2 != 0:
            yield odd_number
            print(f"Yielding odd number: {odd_number}")