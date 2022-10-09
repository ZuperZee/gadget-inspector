import { ModbusBitData, ModbusBitTable } from "@components/ModbusBitTable";
import { ModbusNumericalData, ModbusTable } from "@components/ModbusTable";
import { Button } from "@components/ui/Button";
import { Input } from "@components/ui/Input";
import { Select } from "@components/ui/Select";
import { invoke } from "@tauri-apps/api";
import { Component, createMemo, createSignal, Show } from "solid-js";

interface ModbusData {
  ModbusNumericalData: ModbusNumericalData;
  ModbusBitData: ModbusBitData;
}

async function readModbusAddress({
  socketAddress,
  slaveId,
  address,
  quantity,
  functionCode,
}: {
  socketAddress: string;
  slaveId: number;
  address: number;
  quantity: number;
  functionCode: number;
}) {
  return await invoke<ModbusData>("read_modbus_address_command", {
    socketAddress,
    slaveId,
    address,
    quantity,
    functionCode,
  });
}

const App: Component = () => {
  const [socketAddress, setSocketAddress] =
    createSignal<string>("127.0.0.1:5503");
  const [slaveId, setSlaveId] = createSignal(0);
  const [address, setAddress] = createSignal(0);
  const [functionCode, setFunctionCode] = createSignal(3);
  const [quantity, setQuantity] = createSignal(5);
  const [modbusData, setModbusData] = createSignal<ModbusData>();

  const isBit = createMemo(() => [1, 2].includes(functionCode()));

  return (
    <div>
      <div class="max-w-xs p-2">
        <Input
          id="socket-address"
          labelText="Socket address"
          type="text"
          placeholder="Socket address"
          onChange={(e) => setSocketAddress(e.currentTarget.value)}
          value={socketAddress()}
          autocomplete="url"
        />
        <Input
          id="slave-id"
          labelText="Slave id"
          type="number"
          min={0}
          max={255}
          placeholder="Slave"
          onChange={(e) => setSlaveId(e.currentTarget.valueAsNumber)}
          value={slaveId()}
        />
        <Input
          id="address"
          type="number"
          min={0}
          max={65_535}
          labelText="Modbus address"
          placeholder="Address"
          onChange={(e) => setAddress(Number(e.currentTarget.value))}
          value={address()}
        />
        <Select
          id="function-code"
          labelText="Function code"
          items={[
            { value: 1, text: "(1) Coils" },
            { value: 2, text: "(2) Discrete inputs" },
            { value: 3, text: "(3) Holding registers" },
            { value: 4, text: "(4) Input registers" },
          ].map((a) =>
            a.value === functionCode() ? { ...a, selected: true } : a
          )}
          onChange={(a) => {
            setFunctionCode(Number(a.currentTarget.value));
          }}
        />
        <Input
          id="quantity"
          type="number"
          labelText="Quantity"
          placeholder="Quantity"
          onChange={(e) => setQuantity(Number(e.currentTarget.value))}
          value={quantity()}
        />
        <Button
          onClick={() => {
            readModbusAddress({
              socketAddress: socketAddress(),
              slaveId: slaveId(),
              address: address(),
              quantity: quantity(),
              functionCode: functionCode(),
            })
              .then((res) => setModbusData(res))
              .catch((error) => {
                console.error(error);
                setModbusData();
              });
          }}
        >
          Read modbus
        </Button>
      </div>
      <Show
        when={isBit()}
        fallback={
          <ModbusTable modbusData={modbusData()?.ModbusNumericalData} />
        }
      >
        <ModbusBitTable modbusData={modbusData()?.ModbusBitData} />
      </Show>
    </div>
  );
};

export default App;
