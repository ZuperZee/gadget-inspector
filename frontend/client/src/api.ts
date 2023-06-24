import { ModbusBitData } from "@components/ModbusBitTable";
import { ModbusNumericalData } from "@components/ModbusTable";
import { invoke } from "@tauri-apps/api";

export interface ModbusData {
  ModbusNumericalData: ModbusNumericalData;
  ModbusBitData: ModbusBitData;
}

export async function readModbusAddress({
  socketAddress,
  slaveId,
  address,
  quantity,
  functionCode,
  isByteSwap,
  isWordSwap,
}: {
  socketAddress: string;
  slaveId: number;
  address: number;
  quantity: number;
  functionCode: number;
  isByteSwap: boolean;
  isWordSwap: boolean;
}) {
  return await invoke<ModbusData>("read_modbus_address_command", {
    socketAddress,
    slaveId,
    address,
    quantity,
    functionCode,
    isByteSwap,
    isWordSwap,
  });
}
