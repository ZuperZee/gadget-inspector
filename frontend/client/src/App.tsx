import { ModbusBitTable } from "@components/ModbusBitTable";
import { ModbusTable } from "@components/ModbusTable";
import { Button } from "@components/ui/Button";
import { Input } from "@components/ui/Input";
import { Select } from "@components/ui/Select";
import { Component, createMemo, createSignal, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { ModbusData, readModbusAddress } from "./api";

const App: Component = () => {
  const [socketAddress, setSocketAddress] =
    createSignal<string>("127.0.0.1:5503");
  const [slaveId, setSlaveId] = createSignal(0);
  const [address, setAddress] = createSignal(0);
  const [functionCode, setFunctionCode] = createSignal(3);
  const [quantity, setQuantity] = createSignal(5);
  const [isByteSwap, setIsByteSwap] = createSignal(false);
  const [isWordSwap, setIsWordSwap] = createSignal(false);
  const [modbusState, setModbusState] = createStore<{
    isLoading: boolean;
    errorMessage?: string;
    modbusData?: ModbusData;
  }>({ isLoading: false, errorMessage: undefined, modbusData: undefined });

  const isBit = createMemo(() => [1, 2].includes(functionCode()));

  const readModbus = () => {
    setModbusState({ isLoading: true });
    readModbusAddress({
      socketAddress: socketAddress(),
      slaveId: slaveId(),
      address: address(),
      quantity: quantity(),
      functionCode: functionCode(),
      isByteSwap: isByteSwap(),
      isWordSwap: isWordSwap(),
    })
      .then((res) =>
        setModbusState({
          isLoading: false,
          errorMessage: undefined,
          modbusData: res,
        })
      )
      .catch((error) => {
        console.error(error);
        setModbusState({
          isLoading: false,
          errorMessage: error,
          modbusData: undefined,
        });
      });
  };

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
        <label class="relative mt-2 flex cursor-pointer items-center">
          <input
            type="checkbox"
            value={String(isByteSwap())}
            class="peer sr-only"
            onChange={(e) => setIsByteSwap(e.currentTarget.checked)}
          />
          <div class="peer h-6 w-11 rounded-full bg-gray-200 after:absolute after:top-[2px] after:left-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300" />
          <span class="ml-3 text-sm font-medium text-gray-900">Byte swap</span>
        </label>
        <label class="relative mt-2 flex cursor-pointer  items-center">
          <input
            type="checkbox"
            value={String(isWordSwap())}
            class="peer sr-only"
            onChange={(e) => setIsWordSwap(e.currentTarget.checked)}
          />
          <div class="peer h-6 w-11 rounded-full bg-gray-200 after:absolute after:top-[2px] after:left-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300" />
          <span class="ml-3 text-sm font-medium text-gray-900">Word swap</span>
        </label>
        <Button onClick={readModbus} disabled={modbusState.isLoading}>
          <Show when={!modbusState.isLoading} fallback="Loading...">
            Read modbus
          </Show>
        </Button>
      </div>
      <Show when={modbusState.errorMessage} keyed>
        {(err) => <div class="my-2 ml-2 text-red-500">{err}</div>}
      </Show>
      <Show
        when={isBit()}
        fallback={
          <ModbusTable
            modbusData={modbusState.modbusData?.ModbusNumericalData}
          />
        }
      >
        <ModbusBitTable modbusData={modbusState.modbusData?.ModbusBitData} />
      </Show>
    </div>
  );
};

export default App;
