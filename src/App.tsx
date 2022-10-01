import { ModbusBitData, ModbusBitTable } from "@components/ModbusBitTable";
import { ModbusData, ModbusTable } from "@components/ModbusTable";
import { Button } from "@components/ui/Button";
import { Input } from "@components/ui/Input";
import { Select } from "@components/ui/Select";
import { invoke } from "@tauri-apps/api";
import { Component, createMemo, createSignal, Show } from "solid-js";

async function readModbusAddress({
  socketAddress,
  address,
  quantity,
  functionCode,
}: {
  socketAddress: string;
  address: number;
  quantity: number;
  functionCode: number;
}) {
  return await invoke<ModbusData>("read_modbus_address_command", {
    socketAddress,
    address,
    quantity,
    functionCode,
  });
}

async function readModbusBitAddress({
  socketAddress,
  address,
  quantity,
  functionCode,
}: {
  socketAddress: string;
  address: number;
  quantity: number;
  functionCode: number;
}) {
  return await invoke<ModbusBitData>("read_modbus_bit_address_command", {
    socketAddress,
    address,
    quantity,
    functionCode,
  });
}

const App: Component = () => {
  const [socketAddress, setSocketAddress] =
    createSignal<string>("127.0.0.1:5503");
  const [address, setAddress] = createSignal(0);
  const [functionCode, setFunctionCode] = createSignal(3);
  const [quantity, setQuantity] = createSignal(5);
  const [modbusData, setModbusData] = createSignal<ModbusData>();
  const [modbusBitData, setModbusBitData] = createSignal<ModbusBitData>();

  const isBit = createMemo(() => [1, 2].includes(functionCode()));

  return (
    <div class="flex h-screen flex-col">
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
        id="address"
        type="number"
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
        onClick={async () => {
          if (isBit()) {
            const res = await readModbusBitAddress({
              socketAddress: socketAddress(),
              address: address(),
              quantity: quantity(),
              functionCode: functionCode(),
            });

            console.log(res);
            setModbusBitData(res);
          } else {
            const res = await readModbusAddress({
              socketAddress: socketAddress(),
              address: address(),
              quantity: quantity(),
              functionCode: functionCode(),
            });

            setModbusData(res);
          }
        }}
      >
        Read modbus
      </Button>
      <Show when={isBit()} fallback={<ModbusTable modbusData={modbusData()} />}>
        <ModbusBitTable modbusData={modbusBitData()} />
      </Show>
    </div>
  );
};

export default App;
