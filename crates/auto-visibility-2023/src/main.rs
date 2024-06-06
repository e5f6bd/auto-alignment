use std::path::PathBuf;

struct Opts {
    config_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    // // improving visibility
    // start = time.time()
    // time.sleep(2)
    // shutter(ser_s, 0, "close") // signal light is port 0
    // shutter(ser_s, 1, "close") // LO light is port 1
    // shutter(ser_s, 2, "close")
    // shutter(ser_s, 3, "close")
    // wheel(ser_w, 6) // Attention: must weaken signal light
    // baseLine = Ave(measured_parameter)
    // time.sleep(2)
    // shutter(ser_s, 2, "open")
    // shutter(ser_s, 0, "open")
    // time.sleep(1)
    // ave1 = Ave(measured_parameter)
    // time.sleep(m_time)
    // Input1 = ave1-baseLine
    // shutter(ser_s, 1, "open")    
    // shutter(ser_s, 0, "close")
    // time.sleep(1)
    // ave2 = Ave(measured_parameter)
    // Input2 = ave2-baseLine
    // shutter(ser_s, 0, "open")
    // time.sleep(1)
    // 
    // 
    // // COM_s = "COM3" // for optical shutter
    // // COM = "COM10" // for improving visibility
    // bitRate = 115200
    // device_name = "Dev1/ai0"
    // min_set, max_set, m_time= -3, 3, 0.5
    // measured_parameter = (device_name, min_set, max_set, m_time)
    // e = 1e-05
    // move_p = 10 //pulses
    // constantA = 2.28
    // constantB = 2.85
    // constantC = 1.33
    // constantD = 2.8
    // constant = np.array([constantA, constantB, constantC, constantD])
    // 
    // A = []
    // B = []
    // C = []
    // D = []
    // Z = []
    // T = []
    // time.sleep(2)
    // shutter(ser_s, 3, "close")
    // shutter(ser_s, 0, "close") // signal light is port 0
    // shutter(ser_s, 1, "close") // LO light is port 1
    // baseLine = Ave(measured_parameter)
    // time.sleep(m_time)
    // shutter(ser_s, 0, "open")
    // time.sleep(1)
    // ave1 = Ave(measured_parameter)
    // time.sleep(m_time)
    // Input1 = ave1-baseLine
    // shutter(ser_s, 1, "open")    
    // shutter(ser_s, 0, "close")
    // time.sleep(1)
    // ave2 = Ave(measured_parameter)
    // Input2 = ave2-baseLine
    // shutter(ser_s, 0, "open")
    // time.sleep(1)
    // 
    // vis = Vis(Input1, Input2, baseLine, measured_parameter)
    // if vis < e {
    //     print("Please improve this visiblity")
    //     exit(0)
    // }
    // t = datetime.datetime.now()
    // t = t.strftime('%H%M%S')
    // dotmap = Measurement(measured_parameter)
    // df = pd.DataFrame(dotmap, columns=['Time', 'Voltage'])
    // np.savetxt(r'\Users\kawas\Desktop\Jupyter\auto-alignment_testcode\20240125\visibility\f'+t+'initialWave.csv', df, delimiter=',')
    // 
    // rotation = np.zeros(4)
    // end = time.time()
    // Time = end - start
    // 
    // A.append(rotation[0])
    // B.append(rotation[1])
    // C.append(rotation[2])
    // D.append(rotation[3])
    // Z.append([vis])
    // T.append(Time)
    // i = 1
    // flag = False
    // while True {
    //     if flag {
    //         break
    //     }
    //     print(f"{i} times")
    //     //----グラディエント測定----
    //     grad = Gradient(Input1, Input2, baseLine, move_p, constant, ser1, measured_parameter)
    //     da = grad[0]
    //     db = grad[1]
    //     dc = grad[2]
    //     dd = grad[3]
    //     absgrad = list(map(abs, grad))
    //     index = absgrad.index(max(absgrad))
    //     if abs(da) < e and abs(db) < e and abs(dc) < e and abs(dd) < e {
    //         print("Program finished") //終了条件
    //         break
    //     }
    //     rate = np.full(4, 1.0)
    //     for k in range(1, 4) {
    //         rate[k+index-4] = abs(grad[index+k-4]/grad[index])
    //     }
    // 
    //     R = []
    //     result = []
    //     step_size = int(16-(2*(i-1))) //pulses
    //     step_size1 = step_size 
    // 
    //     r = 0
    //     dirr = 1
    //     R.append(r)
    //     Opt = Vis(Input1, Input2, baseLine, measured_parameter)
    //     result.append(Opt)
    //     if Opt > 99 {
    //         break
    //     } else {
    //         print("Next Optimization!!")
    //     }
    // 
    //     while True {
    //         dire = np.array([1,1,1,1])
    //         movement = np.full(4, step_size1)
    //         for j in range(4) {
    //             if grad[j] > 0 {
    //                 dire[j] = 1
    //                 movement[j] = step_size1*constant[j]
    //             } else {
    //                 dire[j] = -1
    //             }
    //             movement[j] = movement[j]*rate[j]
    //             move(ser1, dire[j], movement[j], j)
    //             rotation[j] = rotation[j] + int(dire[j]*step_size1*rate[j])
    //         }
    // 
    //         r = r + dirr*step_size1
    //         A.append(rotation[0])
    //         B.append(rotation[1])
    //         C.append(rotation[2])
    //         D.append(rotation[3])
    //         R.append(r)
    //         time.sleep(0.2)
    //         Temp = Vis(Input1, Input2, baseLine, measured_parameter)
    //         result.append(Temp)
    //         print(f"Visibility: {Temp}")
    //         end = time.time()
    //         Time = end - start
    //         T.append(Time)
    //         //終了条件
    //         if len(R) >= 10 {
    //             break
    //         }
    //         if abs(Opt-Temp) < 0.1 {
    //             break
    //         }
    //         if Opt <= Temp {
    //             Opt = Temp
    //         } else {
    //             dirr = -1 * dirr
    //             step_size1 = step_size1 * 0.9
    //             if step_size1 < 1 {
    //                 break
    //             }
    //             Opt = Temp
    //             grad = -grad
    //         }
    //         if Opt > 99 {
    //             flag = True
    //             print("visibility is over 99%, program finish")
    //             break
    //         }
    // 
    //     N = np.arange(len(R))
    //     data = np.c_[N, R, result]
    //     raw_data2 = pd.DataFrame(data, columns=['N', 'R', 'Visibility'])
    //     np.savetxt(r'\Users\kawas\Desktop\Jupyter\auto-alignment_testcode\20240125\visibility\f'+t+'step'+str(i)+'.csv', raw_data2, delimiter=',')
    //     result.pop(0)
    //     Z.append(result)
    //     if step_size < 2 {
    //         break
    //     }
    //     i = i + 1
    // }
    // 
    // Z = sum(Z, [])
    // n = np.arange(len(Z))
    // print(f"Final Visibility: {Z[-1]}")
    // Result = np.c_[n, T, A, B, C, D, Z]
    // raw_data3 = pd.DataFrame(Result, columns=['N', 'T', 'A', 'B', 'C', 'D', 'Visibility'])
    // end = time.time()
    // Time = end - start
    // np.savetxt(r'\Users\kawas\Desktop\Jupyter\auto-alignment_testcode\20240125\visibility\f' +t + 'Time'+str(round(Time))+ '.csv', raw_data3, delimiter=',')
    // 
    // 
    // print(Time)

    Ok(())
}
