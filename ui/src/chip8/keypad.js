export const makeKeypad = (keysPressed, isQwerty = true) => {
   return isQwerty ? [
    !!keysPressed["x"], // 0
    !!keysPressed["1"], // 1
    !!keysPressed["2"], // 2
    !!keysPressed["3"], // 3
    !!keysPressed["q"], // 4
    !!keysPressed["w"], // 5
    !!keysPressed["e"], // 6
    !!keysPressed["a"], // 7
    !!keysPressed["s"], // 8
    !!keysPressed["d"], // 9
    !!keysPressed["z"], // A
    !!keysPressed["c"], // B
    !!keysPressed["4"], // C
    !!keysPressed["r"], // D
    !!keysPressed["f"], // E
    !!keysPressed["v"], // F
  ] : [
    !!keysPressed["q"], // 0
    !!keysPressed["1"], // 1
    !!keysPressed["2"], // 2
    !!keysPressed["3"], // 3
    !!keysPressed["'"], // 4
    !!keysPressed[","], // 5
    !!keysPressed["."], // 6
    !!keysPressed["p"], // 7
    !!keysPressed["o"], // 8
    !!keysPressed["e"], // 9
    !!keysPressed[";"], // A
    !!keysPressed["j"], // B
    !!keysPressed["4"], // C
    !!keysPressed["p"], // D
    !!keysPressed["u"], // E
    !!keysPressed["k"], // F
  ];
};
