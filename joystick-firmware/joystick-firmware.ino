const int pin_JoystickLX = A0;
const int pin_JoystickLY = A1;
const int pin_JoystickRX = A2;
const int pin_JoystickRY = A3;

// D-PAD + ABXY + START & SELECT + L1 & R1 + L2 & R2
#define NUM_BUTTONS (4 + 4 + 2 + 2 + 2)

int buttons[NUM_BUTTONS] = {
   2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, A4, A5
};

void setup() {
  Serial.begin(115200);

  pinMode(pin_JoystickLX, INPUT);
  pinMode(pin_JoystickLY, INPUT);
  pinMode(pin_JoystickRX, INPUT);
  pinMode(pin_JoystickRY, INPUT);

  for (int i = 0; i < NUM_BUTTONS; i++) {
    pinMode(buttons[i], INPUT_PULLUP);
  }
}

void loop() {  
  uint16_t buttonState = readButtonState();
  uint16_t joystickState[4];
  readJoystickState(joystickState);

  uint16_t startShort = 65535; // 16 1s
  writeShort(startShort);
  writeShort(buttonState);
  for (int i = 0; i < 4; i++) {
    writeShort(joystickState[i]);
  }

  delay(10);
}

void readJoystickState(uint16_t joystickState[4]) {
  joystickState[0] = analogRead(pin_JoystickLX);
  joystickState[1] = analogRead(pin_JoystickLY);
  joystickState[2] = analogRead(pin_JoystickRX);
  joystickState[3] = analogRead(pin_JoystickRY);
}

uint16_t readButtonState() {
  uint16_t buttonState = 0;
  
  for (int i = 0; i < NUM_BUTTONS; i++) {
    if (digitalRead(buttons[i]) == LOW) {
      buttonState |= (1 << i);
    }
  }

  return buttonState;
}

void writeShort(uint16_t value) {
  Serial.write((uint8_t) value);
  Serial.write((uint8_t) (value >> 8));
}
