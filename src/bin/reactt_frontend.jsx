// LEVEL ONE
// You are given a page with a partial implementation of the clock. Your task is to render the missing buttons. The result should look as follows:



// This is the HTML template (note that it's pure HTML, and you might need to convert it to your framework's template syntax):



// <div id="ClockUpdater" class="container">
//   <div class="row">
//     <button id="hours-up-button">&uarr;</button>
//     <button id="minutes-up-button">&uarr;</button>
//   </div>

//   <div class="row">
//     <div id="clock"></div>
//   </div>

//   <div class="row">
//     <button id="hours-down-button">&darr;</button>
//     <button id="minutes-down-button">&darr;</button>
//   </div>
// </div>


// Note: HTML element ids are used for the testing. Please, make sure you are using the correct ones. Otherwise, your test won't run correctly.

// Tests
// Unit tests are provided in the test/level<i>.test.js files for each level. To run the tests, click the blue button 'Run'. You can choose to run the tests In Terminal or in the Structured manner.
// You may use test/sample.test.js to write your own tests, which will also be included in the test runs.
// If you would like to include debugging output to the console in your tests, use the In Terminal option to receive the raw test output.
// Note, that debugging output from your application code you can find in your browser's console.
// When working on a scored certification, partial credit will be granted for each unit test passed, so press Submit often to run tests and receive partial credits for passed tests.

// [execution time limit] 55 seconds

// Please note that if the question needs to run Front-End unit tests, a headless browser might be initialized for every test run, taking an average time of ~20 seconds.

// [memory limit] 1 GB
// LEVEL 2

// You are given a page with a clock. Your task is to implement the logic for buttons to increment and decrement the hours and minutes.

// The time should be displayed in HH:MM 24-hour format. For example, 23:59, 08:00, 07:32.

// The initial state of the clock should be 00:00.

// Actions should be cycled; if you have 23:58 on the clock and press up for hours, the resulting time should be 00:58.

// Minutes and hours should be changed separately.

import { useState } from 'react';
import './index.scss';

const App = () => {
  // We Store as numbers for easy arithmetic
  const [hours, setHours] = useState(0);
  const [minutes, setMinutes] = useState(0);

  const incrementHours = () => {
    setHours((prev) => (prev + 1) % 24);
  };

  const decrementHours = () => {
    setHours((prev) => (prev - 1 + 24) % 24);
  };

  const incrementMinutes = () => {
    setMinutes((prev) => (prev + 1) % 60); 
  };

  const decrementMinutes = () => {
    setMinutes((prev) => (prev - 1 + 60) % 60)
  };

  return (
    <div id="ClockUpdater" className="container">


      {/* Increase Hours */}
      <div className="row">
        <button
          id="hours-up-button"
          className="btn btn-outline-primary col"
          onClick={incrementHours}
        >
          &uarr;
        </button>
      </div>



      {/* Increase Minutes */}
      <div className="row">
        <button
          id="minutes-up-button"
          className="btn btn-outline-primary col"
          onClick={incrementMinutes}
        >
          &uarr;
        </button>
      </div>



    //  {/* Clock Display */}
      <div className="row">
        <div id="clock" className="badge badge-primary col">
          {String(hours).padStart(2, '0')}:
          {String(minutes).padStart(2, '0')}
        </div>
      </div>



      {/* Decrease Hours */}
      <div className="row">
        <button
          id="hours-down-button"
          className="btn btn-outline-primary col"
          onClick={decrementHours}
        >
          &darr;
        </button>
      </div>

      {/* Decrease Minutes */}
      <div className="row">
        <button
          id="minutes-down-button"
          className="btn btn-outline-primary col"
          onClick={decrementMinutes}
        >
          &darr;
        </button>
      </div>
    </div>
  );
};

export default App;