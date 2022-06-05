import { ModbusData } from "@components/ModbusData";
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
    createSignal<string>("127.0.0.1:5502");
  const [address, setAddress] = createSignal(0);
  const [quantity, setQuantity] = createSignal(0);
  const [modbusData, setModbusData] = createSignal<ModbusData>();

  return (
    <div>
      <Input
        type="text"
        placeholder="Socket address"
        onInput={(e) => setSocketAddress(e.currentTarget.value)}
        value={socketAddress()}
      />
      <Input
        type="number"
        placeholder="Address"
        onInput={(e) => setAddress(Number(e.currentTarget.value))}
        value={address()}
      />
      <Input
        type="number"
        placeholder="Quantity"
        onInput={(e) => setQuantity(Number(e.currentTarget.value))}
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
      <ModbusData modbusData={modbusData()} />
    </div>
  );
};

export default App;
