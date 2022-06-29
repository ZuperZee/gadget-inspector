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
  const [modbusData, setModbusData] = createSignal<ModbusData>({
    addresses: [
      0,
      1,
      2,
      3,
      4,
      ...[1, 2, 3, 4, 5].flatMap((a) =>
        Array.from<number>({ length: 100 }).fill(a)
      ),
    ],
    float32: [
      9.183_829_875_491_986e-41, 1.836_751_962_113_754e-40,
      2.755_120_936_678_309_3e-40, 3.673_489_911_242_865e-40,
    ],
    float64: [1.390_713_602_454_213e-309, 2.781_405_984_302_92e-309],
    sint8: [0, 1, 0, 2, 0, 3, 0, 4, 0, 5],
    sint16: [1, 2, 3, 4, 5],
    sint32: [65_538, 131_075, 196_612, 262_149],
    sint64: [281_483_566_841_860, 562_962_838_585_349],
    uint8: [0, 1, 0, 2, 0, 3, 0, 4, 0, 5],
    uint16: [
      1,
      2,
      3,
      4,
      5,
      ...[1, 2, 3, 4, 5].flatMap((a) =>
        Array.from<number>({ length: 100 }).fill(a)
      ),
    ],
    uint32: [65_538, 131_075, 196_612, 262_149],
    uint64: [281_483_566_841_860, 562_962_838_585_349],
    ascii: ["N", "S", "N", "T", "N", "U", "N", "V", "N", "W"],
  });

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
