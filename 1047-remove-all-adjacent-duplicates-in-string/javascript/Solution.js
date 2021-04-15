var removeDuplicates = function (S) {
  const stack = [];
  for (const ch of S) {
    if (stack.length && stack[stack.length - 1] === ch) {
      stack.pop();
    } else {
      stack.push(ch);
    }
  }
  return stack.join('');
};