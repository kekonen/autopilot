  

     
/*
 RC PulseIn Serial Read out
 By: Nick Poole
 SparkFun Electronics
 Date: 5
 License: CC-BY SA 3.0 - Creative commons share-alike 3.0
 use this code however you'd like, just keep this license and
 attribute. Let me know if you make hugely, awesome, great changes.
 */
#include <Servo.h>
Servo myservo1;
Servo myservo2;
Servo myservo3;

int ch1; // Here's where we'll keep our channel values
int ch2;
int ch3;

int val1;
int val2;
int val3;

void setup() {

  pinMode(5, INPUT); // Set our input pins as such
  pinMode(6, INPUT);
  pinMode(7, INPUT);

  myservo1.attach(9);  // attaches the servo on pin 9 to the servo object
  myservo2.attach(10);
  myservo3.attach(11);


  Serial.begin(9600); // Pour a bowl of Serial

}

void loop() {

  ch1 = pulseIn(5, HIGH, 25000); // Read the pulse width of  //1070 - 1811
  ch2 = pulseIn(6, HIGH, 25000); // each channel // 1201 - 1822
  ch3 = pulseIn(7, HIGH, 25000);     //       1120 - 1855

  val1 = map(ch1, 1070, 1811, 0, 180);
  myservo1.write(val1);  

  val2 = map(ch2, 1201, 1822, 0, 180);
  myservo2.write(val2);  

  val3 = map(ch3, 1120, 1855, 0, 180);
  myservo3.write(val3); 

//  Serial.print("Channel 1:"); // Print the value of 
//  Serial.println(ch1);        // each channel
//
//  Serial.print("Channel 2:");
//  Serial.println(ch2);
//
//  Serial.print("Channel 3:");
//  Serial.println(ch3);
  

//  delay(100); // I put this here just to make the terminal 
              // window happier
}


