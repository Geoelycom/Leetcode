const buttons = document.getElementsByTagName("button");
const boxes = document.getElementsByClassName("box");

const arr = Array.prototype.map.call(boxes, box => box.innerHTML);
buttons[0].onclick = function() {
  arr.push(arr.shift());
  Array.prototype.forEach.call(boxes, (box, ind) => box.innerHTML = arr[ind]);
}

buttons[1].onclick = function() {
  arr.unshift(arr.pop());
  Array.prototype.forEach.call(boxes, (box, ind) => box.innerHTML = arr[ind]);
}

// HTML
// <div>
//   <div class="left-shift-button">
//     <button><<</button>
//   </div>
//   <div class="boxes">
//     <div class="box">1</div>
//     <div class="box">2</div>
//     <div class="box">3</div>
//     <div class="box">4</div>
//     <div class="box">5</div>
//   </div>
//   <div class="right-shift-button">
//     <button>>></button>
//   </div>
// </div>


// CSS

// .box {
//   float: left;
//   width: 20%;
//   font-size: 40px;
// }
// button {
//   font-size: 40px;
// }
// .left-shift-button, .right-shift-button, .boxes {
//   float: left;
//   width: 33.33%;
//   text-align: center;
// }