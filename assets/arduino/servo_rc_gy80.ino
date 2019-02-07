  

     
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
#include <Wire.h>
#include <ADXL345.h>

//Pitch_roll
const float alpha = 0.5;

double fXg = 0;
double fYg = 0;
double fZg = 0;

ADXL345 acc;
// end Pitch_roll

Servo myservo1;
Servo myservo2;
Servo myservo3;

int ch1; // Here's where we'll keep our channel values
int ch2;
int ch3;

int val1;
int val2;
int val3;

float s;
bool up;
int gyro_counter;

void setup() {

  acc.begin();
  gyro_counter = 0;

  pinMode(32, INPUT); // Set our input pins as such
  pinMode(35, INPUT);
  pinMode(34, INPUT);

  myservo1.attach(13);  // attaches the servo on pin 9 to the servo object
  myservo2.attach(12);
  myservo3.attach(14);
//  s = 0;
//  up = true;

  Serial.begin(9600); // Pour a bowl of Serial

}

void loop() {
// ------ COOOL
//  ch1 = pulseIn(32, HIGH, 25000); // Read the pulse width of  //1070 - 1811
//  ch2 = pulseIn(35, HIGH, 25000); // each channel // 1201 - 1822
//  ch3 = pulseIn(34, HIGH, 25000);     //       1120 - 1855
//
//  val1 = map(ch1, 1070, 1811, 0, 180);
//  myservo1.write(val1);  
//
//  val2 = map(ch2, 1201, 1822, 0, 180);
//  myservo2.write(val2);  
////
//  val3 = map(ch3, 1120, 1855, 0, 180);
//  myservo3.write(val3); 
// ------ END COOOL

  if (gyro_counter == 00) {
    double pitch, roll, Xg, Yg, Zg;
    acc.read(&Xg, &Yg, &Zg);
  
    //Low Pass Filter
    fXg = Xg * alpha + (fXg * (1.0 - alpha));
    fYg = Yg * alpha + (fYg * (1.0 - alpha));
    fZg = Zg * alpha + (fZg * (1.0 - alpha));
  
    //Roll & Pitch Equations
    roll  = (atan2(-fYg, fZg)*180.0)/M_PI;
    pitch = (atan2(fXg, sqrt(fYg*fYg + fZg*fZg))*180.0)/M_PI;
  
    Serial.print(pitch);
    Serial.print(":");
    Serial.println(roll);
    
    myservo1.write(pitch);  
  
    myservo2.write(roll); 

    delay(10);
  }
  

//    if (s >= 180) {
//      up = false;
//    }
//    if (s <= 0) {
//      up = true;
//    }
//
//    if (up){
//      s+=0.001;
//    } else {
//      s-=0.001;
//    }
//
//    myservo1.write(s);
//    myservo2.write(s);
//    myservo3.write(s); 
//  Serial.print("Channel 1:"); // Print the value of 
//  Serial.print(ch1);        // each channel
//  Serial.print(",");
//  Serial.println(val1);
//  
//  Serial.print("Channel 2:");
//  Serial.print(ch2);
//  Serial.print(",");
//  Serial.println(val2);
//
//  Serial.print("Channel 3:");
//  Serial.print(ch3);
//  Serial.print(",");
//  Serial.println(val3);

//  Serial.print(ch1);
//  Serial.print("  ");
//  Serial.print(ch2);
//  Serial.print("  ");
//  Serial.println(ch3);
  
//  delay(100); // I put this here just to make the terminal 
              // window happier
}
