import { ModbusData, ModbusTable } from "@components/ModbusTable";
import { Button } from "@components/ui/Button";
import { Input } from "@components/ui/Input";
import { invoke } from "@tauri-apps/api";
import { Component, createSignal } from "solid-js";

async function readModbus({
  socketAddress,
  address,
  quantity,
}: {
  socketAddress: string;
  address: number;
  quantity: number;
}) {
  return await invoke<ModbusData>("read_modbus_command", {
    socketAddress,
    address,
    quantity,
  });
}

const App: Component = () => {
  const [socketAddress, setSocketAddress] =
    createSignal<string>("127.0.0.1:5503");
  const [address, setAddress] = createSignal(0);
  const [quantity, setQuantity] = createSignal(5);
  const [modbusData, setModbusData] = createSignal<ModbusData>();

  return (
    <div class="flex h-screen flex-col">
      <Input
        type="text"
        placeholder="Socket address"
        onChange={(e) => setSocketAddress(e.currentTarget.value)}
        value={socketAddress()}
      />
      <Input
        type="number"
        placeholder="Address"
        onChange={(e) => setAddress(Number(e.currentTarget.value))}
        value={address()}
      />
      <Input
        type="number"
        placeholder="Quantity"
        onChange={(e) => setQuantity(Number(e.currentTarget.value))}
        value={quantity()}
      />
      <Button
        onClick={async () => {
          const res = await readModbus({
            socketAddress: socketAddress(),
            address: address(),
            quantity: quantity(),
          });

          console.log(res);
          setModbusData(res);
        }}
      >
        Read modbus
      </Button>
      <ModbusTable modbusData={modbusData()} />
    </div>
  );
};

export default App;
