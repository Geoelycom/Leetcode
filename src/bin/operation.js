// Operations
// The program starts with an empty container.

// ADD <value> should add the specified integer value to the container.

// DELETE <value> should attempt to remove the specified integer value from the container. If the value is present in the container, remove it and return true, otherwise, return false.

// GET_MEDIAN should return the median integer - the integer value in the middle of the sequence after all integer value stored in the container are sorted from smallest to largest. If the length of the sequence is even, the leftmost integer from the two middle integers should be returned. If the container is empty, this method should raise a runtime exception.

// [execution time limit] 6 seconds

// [memory limit] 1 GB


/**
 * A container of integers that should support
 * addition, removal, and search for the median integer
 */
class Container {
  constructor() {
      this.values = []; // Maintain sorted order
  }

  /**
   * Adds the specified value to the container
   * @param {number} value
   */
  add(value) {
      let index = this.values.findIndex((v) => v > value);
      if (index === -1) {
          this.values.push(value);
      } else {
          this.values.splice(index, 0, value);
      }
  }
  /**
   * Deletes one item of the specified value from the container
   * @param {number} value
   * @return {boolean} true if value is deleted, false otherwise.
   */
  delete(value) {
      let index = this.values.indexOf(value);
      if (index !== -1) {
          this.values.splice(index, 1);
          return true;
      }
      return false;
  }

  /**
   * Returns the median value of the container
   * @return {number} The median of the sorted numbers
   * @throws {Error} If the container is empty
   */
  getMedian() {
      if (this.values.length === 0) {
          throw new Error("Container is Empty");
      }
      let mid = Math.floor((this.values.length - 1) / 2); // Ensure leftmost median for even lengths
      return this.values[mid];
  }
}

module.exports = Container;



