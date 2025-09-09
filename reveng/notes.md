Connection sequence:

- Connect to device
- Discover services/characteristics
- Read 0x1501 (Version)
- Check for update (TODO: document update process)
- Enable notifications for 0x1500 (Battery Level)
- Enable notifications for 0x150B (Device Notification)
- Receive notification Value: 530039062bf8
- Send command Value: ff
- Send command Value: b014000000000000000000ff00000000000000ff
- Send command Value: 5007000000000000000000000000000000
- Receive notification Value: b1010000
- Receive notification Value: f1010000
- Receive notification Value: f301010000000000000000000000000000000000
- Receive notification Value: f302010000000000000000000000000000000000
- Send command Value: bf6464ffa00000
- Receive notification Value: f200000000000000000000000000000000000000
- Receive notification Value: f40000ffff000000000000000000


Save waveform to device:
selected chA slot 2:
- Sent command Value: a002030014c41e
- Received notification Value: a10300
- Sent command Value: a200140019191e32234b28642d4b323237193c00
- Sent command Value: a2010a00
- Received notification Value: a303

## Attribute Table
| Service UUID | Service Name               | Characteristic UUID | Characteristic Name        | Properties           |
|--------------|----------------------------|---------------------|----------------------------|----------------------|
| 0x1800       | GAP Service                | 0x2A00              | Device Name                | Read, Write          |
| 0x1800       | GAP Service                | 0x2A01              | Appearance                 | Read                 |
| 0x1800       | GAP Service                | 0x2A04              | Peripheral Preferred Connection Parameters | Read |
| 0x1800       | GAP Service                | 0x2AA6              | Central Address Resolution | Read                 |
| 0x1801       | GATT Service               | 0x2A05              | Service Changed            | Indicate             |
| 0x180A       | Device Information Service | 0x1500              | Battery Level              | Read, Notify         |
| 0x180A       | Device Information Service | 0x1501              | Version                    | Read                 |
| 0x180A       | Device Information Service | 0x1502              | Bluetooth MAC              | Read                 |
| 0x180C       | Pulse Host                 | 0x150A              | Device Command             | Write                |
| 0x180C       | Pulse Host                 | 0x150B              | Device Notification        | Notify               |
| 0x2003       | Unknown Service            | 0x0007              | Unknown Characteristic     | Write                |
| 0x2003       | Unknown Service            | 0x0008              | Unknown Characteristic     | Read, Notify         |
| 0x2004       | Unknown Service            | 0x0009              | Unknown Characteristic     | Read                 |
| 0xFE59       | Secure DFU Service         | 8ec90003-f315-4f60-9fb8-838830daea50 | Buttonless DFU | Write, Indicate |
//!
//! | Service                     | Characteristic                      | Properties   | Value Size      | Description                                               |      Example Value |
//! |-----------------------------|-------------------------------------|--------------|-----------------|-----------------------------------------------------------|--------------------|
//! | 0x1800 - Generic Access     | 0x2A00 - Device Name                | Read, Write  | Up to 248 bytes | Device name                                               |        "47L121000" |
//! | 0x1800 - Generic Access     | 0x2A01 - Appearance                 | Read         | 2 bytes         | Appearance of the device                                  |             0x0000 |
//! | 0x1800 - Generic Access     | 0x2A04 - Peripheral Preferred Connection Parameters | Read | 8 bytes | The preferred connection parameters of the Peripheral     | 0x1000280000009001 |
//! | 0x1800 - Generic Access     | 0x2AA6 - Central Address Resolution | Read         | 1 byte          | Defines whether the device supports privacy with address resolution. |    0x01 |
//! | 0x1801 - Generic Attribute  | 0x2A05 - Service Changed            | Indicate     | 4 bytes         | Service Changed                                           |                    |
//! | 0x180A - Device Information | 0x1500 - Battery Power              | Read, Notify | 1 byte          | Battery percentage (0-100)                                |               0x5F |
//! | 0x180A - Device Information | 0x1501 - Version                    | Read         | 2 bytes         | Version, but app code suggests maybe also device type?    |             0x0703 |
//! | 0x180A - Device Information | 0x1502 - Bluetooth MAC              | Read         | 6 bytes         | MAC address for the BT interface                          |     0xC0C939062BF8 |
//! | 0x180C - Pulse Host         | 0x150A - Device Command             | Write        | Up to 20 bytes  | All commands are input in this characteristic             |                    |
//! | 0x180C - Pulse Host         | 0x150B - Device Notification        | Notify       | Up to 20 bytes  | All response messages are returned in this characteristic |                    |
//! | 0x2003 - Unknown Service    | 0x0007 - Unknown Characteristic     | Write        | Unknown         | Unknown Characteristic                                    |                    |
//! | 0x2003 - Unknown Service    | 0x0008 - Unknown Characteristic     | Read, Notify | Unknown         | Unknown Characteristic                                    |                    |
//! | 0x2004 - Unknown Service    | 0x0009 - Unknown Characteristic     | Read         | 2 bytes         | Unknown Characteristic                                    |               0x00 |
//! | 0xFE59 - Secure DFU Service | 8ec90003-f315-4f60-9fb8-838830daea50 - Buttonless DFU | Write, Indicate | Unknown | Buttonless DFU                               |                    | 
//! 


## Device Commands

| Command | Data Length  | Description                                                                         | Data Example                                                  |
|---------|--------------|-------------------------------------------------------------------------------------|---------------------------------------------------------------|
| 0x01    | 1 byte       | Sent when triggering pawprint pairing                                               | 0x01                                                          |
| 0x08    | 2 byte       | Unknown                                                                             | 0x08 01                                                       |
| 0x50    | 17 bytes     | Unknown                                                                             | 0x50 07 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00          |
| 0x60    | 4 bytes      | Might be set color for connected button?                                            | 0x60 01 70 07                                                 |
| 0xB0    | 20 bytes     | Sets the intensity and waveform data for both output channels.                      | 0xB0 00 00 00 00 00 00 00 00 00 00 FF 00 00 00 00 00 00 00 FF |
| 0xBF    | 7 bytes      | Sets the intensity limits and waveform balance parameters for both output channels. | 0xBF 64 64 A0 A0 00 00                                        |
| 0xFE    | 1 byte       | Sent after "Clear Accessory Data" clicked. The rcv'd F1, F3, F2, F4                 | 0xFE                                                          |
| 0xFF    | 1 byte       | get all pawprint settings?                                                       | 0xFF                                                          |

## Device Notification

| Notification | Data Length | Description                                                                         | Data Example         |
|--------------|-------------|-------------------------------------------------------------------------------------|----------------------|
| 0x02         | 5 bytes     | Response to 0x01 command?                                                           | 0x02 01 01 0a 01     |
| 0x03         | Unknown     | Unknown                                                                             |                      |
| 0x04         | Unknown     | Unknown                                                                             |                      |
| 0x09         | 2 bytes     | Response to 0x08 command? (dupe body data)                                          | 0x09 01              |
| 0x0C         | Unknown     | Unknown                                                                             |                      |
| 0x0D         | Unknown     | Unknown                                                                             |                      |
| 0x0E         | Unknown     | Unknown                                                                             |                      |
| 0x0F         | Unknown     | Unknown                                                                             |                      |
| 0x10         | Unknown     | Unknown                                                                             |                      |
| 0x21         | Unknown     | Unknown                                                                             |                      |
| 0x29         | Unknown     | Unknown                                                                             |                      |
| 0x31         | Unknown     | Unknown                                                                             |                      |
| 0x41         | Unknown     | Unknown                                                                             |                      |
| 0x51         | 4 bytes     | Last byte is same as battery level                                                  | 0x51 00 10 5B        |
| 0x53         | 6 bytes     | Unknown                                                                             | 0x53 00 39 06 2B F8  |
| 0x61         | 4 bytes     | Response to 0x60 command? (dupe body data)                                          | 0x61 01 70 07        |
| 0x70         | Unknown     | Unknown                                                                             |                      |
| 0x99         | Unknown     | Unknown                                                                             |                      |
| 0xA1         | Unknown     | Unknown                                                                             |                      |
| 0xA3         | Unknown     | Unknown                                                                             |                      |
| 0xA4         | Unknown     | Unknown                                                                             |                      |
| 0xAE         | Unknown     | Unknown                                                                             |                      |
| 0xB1         | 4 bytes     | Returns the channel intensities when they are changed.                              | 0xB1 0F 00 00        |
| 0xBD         | Unknown     | Unknown                                                                             |                      |
| 0xBE         | 6 bytes     | Returns the intensity limits and waveform balance parameters when they are changed. |                      |
| 0xC9         | Unknown     | Unknown                                                                             |                      |
| 0xD1         | Unknown     | Unknown                                                                             |                      |
| 0xE0         | 3 bytes     | Error output message.                                                               | 0xE0 01 00           |
| 0xE2         | Unknown     | Unknown                                                                             |                      |
| 0xED         | Unknown     | Unknown                                                                             |                      |
| 0xF1         | 4 bytes     | Unknown. Might be dupe of 0xB1                                                      | 0xF1 01 00 00        |
| 0xF2         | 20 bytes    | Unknown. Rcv'd after 0xBF and 0xFF. Contains trigger action config                                    | 0xF2 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 |
| 0xF3         | 20 bytes    | Unknown. Rcv'd after 0xBF                                                           | 0xF3 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 |
| 0xF4         | 14 bytes    | Unknown. Rcv'd after 0xBF                                                           | 0xF4 00 00 FF FF 00 00 00 00 00 00 00 00 00 |

## Commands

### 0x01 - Pawprint Pairing

Sent when hitting the pawprint pairing button in the app.

### 0x20 - Trigger Action Configuration

Writes the configuration for a trigger action.

### 0x28 - Delete Trigger

2 bytes

```
+-----+----------------+
|   0 | 0x28           |
+-----+----------------+
|   1 | Trigger Number |
+-----+----------------+
```

### 0x30 - Unknown

Sent when hitting "Reaload settings" on pawprint in app.

btn1: 0x30 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
btn2: 0x30 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00

### 0x60 - ??

Sent when changing sholder lights on pawprint in the app.
Sent when hitting "Clear trigger param" on pawprint in the app.

```

btn1
 - black: 0x60 01 70 00
 - gold:  0x60 01 70 01
 - red:   0x60 01 70 02

btn2
 - black: 0x60 02 70 00

```
4 bytes
+--------------------+--------------------+--------------------+--------------------+
| 0x60               | Button number      | Unknown            | Color index        |
+--------------------+--------------------+--------------------+--------------------+
```

Colors:
- 0x00 - Black/Off
- 0x01 - Gold
- 0x02 - Red
- 0x03 - Purple
- 0x04 - Blue
- 0x05 - Cyan
- 0x06 - Green
- 0x07 - White

Clear trigger param:

btn1: 0x60 01 5f
btn2: 0x60 02 5f

```
3 bytes

```
4 bytes
+--------------------+--------------------+--------------------+
| 0x60               | Button number      | Unknown            |
+--------------------+--------------------+--------------------+
```


## Responses

### 0x02 - Pawprint connected?

Recieved after pawprint connects after 0x01 command.

First button:

0x02 01 01 0a 01
0x02 01 01 1e 01
- Recieved after 0x01 command
- app knows button is 30% charged
- app knows it has a firmware update - curr ver 1.0

Second button:

0x02 02 01 14 01

```
5 bytes
+--------------------+--------------------+--------------------+--------------------+--------------------+
| 0x02               | Buton number       | Unknown            | Batery level       | Unknown            |
+--------------------+--------------------+--------------------+--------------------+--------------------+
```

### 0x03 - No pawprint found?

Recieved after 0x01 command when no pawprint is found.


```
1 byte
+--------------------+
| 0x03               |
+--------------------+
```

### 0x31 - Unknown

Recieved after 0x30 command

btn1: 0x31 01 01 14 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
btn2: 0x31 02 01 14 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00

First 3 payload bytes appear to be the same as the 0x02 response.

btn num + unkn + bat lvl + unkn

### 0x61 - Set Color Response

Recieved after 0x60 command.

Contains copy of payload from 0x60 command.

```
4 bytes
+--------------------+--------------------+--------------------+--------------------+
| 0x61               | Button number      | Unknown            | Color index        |
+--------------------+--------------------+--------------------+--------------------+
```

----------

```
Setting up a trigger

3rd trigger btn 1 press
0x30 01 01 03 00 00 00 00 00 00 00 00 00 00 00 00 00

2rd trigger btn 1 press
0x30 01 01 02 00 00 00 00 00 00 00 00 00 00 00 00 00

3rd trigger btn 2 press
0x30 02 01 03 00 00 00 00 00 00 00 00 00 00 00 00 00

3rd trigger btn 2 release
0x30 02 01 03 01 00 00 00 00 00 00 00 00 00 00 00 00

3rd trigger btn 1 press or btn 2 press
0x30 01 01 03 00 00 00 00 00 00 00 00 00 00 00 00 00
0x30 02 01 03 00 00 00 00 00 00 00 00 00 00 00 00 00

3rd trigger btn 1 accel, mode 1, < TH 3
0x30 01 02 00 00 00 03 00 00 00 03 00 00 00 00 00 00

3rd trigger btn 1 accel, mode 1, < TH 7
0x30 01 02 00 00 00 03 00 00 00 07 00 00 00 00 00 00

1st trigger btn 2 accel, mode 1, < TH 7
0x30 02 02 00 00 00 01 00 00 00 07 00 00 00 00 00 00

3rd trigger btn 1 accel, mode 1, > TH 3
0x30 01 02 00 00 00 03 00 01 00 03 00 00 00 00 00 00

3rd trigger btn 1 accel, mode 2, X -14 to 14, Y -24 to 24, Z -14 to 14
0x30 01 02 00 00 00 03 02 f9 07 f4 0c f0 10 00 00 00

3rd trigger btn 1 rand react, activates after 6s to 30s, react within 5s
0x30 01 03 03 00 06 00 1e 00 05 00 00 00 00 00 00 00

3rd trigger btn 1 rand, 2%, 8s cooldown
0x30 01 04 03 04 00 00 00 00 00 00 00 00 00 00 00 

3rd trigger btn 1 ext trig, High Pot, 0.14-0.42, 0.280v
0x30 01 0f 03 00 10 30 20 00 00 00 00 00 00 00 00 00

3rd trigger btn 1 ext trig, High Res, 0.14-0.42, 0.280v
0x30 01 0f 03 01 10 30 20 00 00 00 00 00 00 00 00 00


rand prob has no advanced
ext trig has no advanced

30 = trigger condition

| 0        | 1        | 2        | 3        | 4        | 5        | 6        | 7        | 8        | 9        | 10       | 11       | 12       | 13       | 14       | 15       | 16       |
+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+
| 0x30     | Btn #    | Cond typ | Trig #   | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     | Unkn     |
+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+----------+

```

----------

Reverse engineering notes:

ESTMExecutionData.dataReceive() checks for
- 0xB1 - this.strengthUpdateNew(var1);
- 0xC9 - this.strengthUpdateOld(var1);
- 0xD1 - no handling
- 0xE0 - this.errorOutPut(var1);
  - 3 bytes
  - 0xE0 + unknown + message index
  - messages:
    - 1 - The command type is not understood (the command type is not defined)
    - 2 - The message format is incorrect
    - 3 - This command cannot be executed during accessory gameplay
    - 4 - The object does not exist (you need to get the gameplay configuration information again)
    - 5 - Writing to flash failed
    - 6 - Command queue is full

NewESTMExecutionData.dataReceive() checks for
- 0x02 - stopTimeOut(); analysisAccessoryDeviceData(bArr, 0); setAccessoryNotification(new int[]{0, bArr[1], bArr[2]});
- 0x03 - setAccessoryNotification(new int[]{1, -1, -1});
- 0x04 - stopTimeOut(); analysisAccessoryDeviceData(bArr, 0); setAccessoryNotification(new int[]{0, bArr[1], bArr[2]});
- 0x09
- 0x0C - stuff for bonding
- 0x0D - also for bonding
- 0x0E
- 0x0F - analysisAccessoryDeviceData(bArr, 1);
- 0x10 - analysisAccessoryStatusAndTime(bArr);
- 0x21 - nextRunRuleChangeQueue(); replyResultData(bArr);
- 0x29
- 0x31 - stopTimeOut(); setAccessoryNotification(new int[]{19, -1, -1}); nextRunRuleChangeQueue(); replyTriggerData(bArr);
- 0x41 - replySetting(bArr); nextRunRuleChangeQueue();
- 0x61
- 0x70
- 0x99 - this.loadStatus.setValue(new int[]{var4, var5}); this.remoteUpdateLoadStatus();
- 0xA1
- 0xA3
- 0xA4
- 0xAE
- 0xB1 - this.strengthUpdateNew(var1);
- 0xBD
- 0xC9 - this.strengthUpdateOld(var1);
- 0xD1 - no handling
- 0xE0 - this.errorOutPut(var1);
  - 3 bytes
  - 0xE0 + unknown1 + unknown2
  - unknown2:
    - 2, 3, 4 - setAccessoryNotification(new int[]{2, -1, -1});
    - 48 - this.ruleChanging = false;
    - 7 - this.deviceForbiden.setValue(Boolean.TRUE); disConnected();
- 0xE2
- 0xED
- 0xF1 - analysisAccessoryStatusAndTime(bArr);
- 0xF2 - analysisAccessoryResult(bArr);
- 0xF3 - analysisAccessoryDeviceAndTriggers(bArr);
- 0xF4 - stopTimeOut(); analysisAccessorySetting(bArr);

TODO:
 - Does changing intensity using the physical dial trigger a 0xB1 notification?
