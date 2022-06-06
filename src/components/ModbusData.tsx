import { For } from "solid-js";

export interface ModbusData {
  addresses: number[];

  uint8: number[];
  uint16: number[];
  uint32: number[];
  uint64: number[];

  sint8: number[];
  sint16: number[];
  sint32: number[];
  sint64: number[];

  float32: number[];
  float64: number[];
}

export function ModbusData(props: { modbusData?: ModbusData }) {
  return (
    <div class="flex space-x-4">
      <div>
        <div>Addr</div>
        <For each={props.modbusData?.addresses}>
          {(data) => <div>{data}</div>}
        </For>
      </div>
      <div>
        <div>Binary</div>
        <For each={props.modbusData?.uint16}>
          {(data) => {
            const binaryString = data.toString(2).padStart(16, "0");
            return (
              <div class="whitespace-nowrap">
                {binaryString.slice(0, 8)} {binaryString.slice(8)}
              </div>
            );
          }}
        </For>
      </div>
      <div>
        <div>Uint8</div>
        <div class="flex space-x-4">
          <div>
            <For each={props.modbusData?.uint8.filter((_, i) => i % 2 === 0)}>
              {(data) => <div>{data}</div>}
            </For>
          </div>
          <div>
            <For each={props.modbusData?.uint8.filter((_, i) => i % 2 === 1)}>
              {(data) => <div>{data}</div>}
            </For>
          </div>
        </div>
      </div>
      <div>
        <div>Sint8</div>
        <div class="flex space-x-4">
          <div>
            <For each={props.modbusData?.sint8.filter((_, i) => i % 2 === 0)}>
              {(data) => <div>{data}</div>}
            </For>
          </div>
          <div>
            <For each={props.modbusData?.sint8.filter((_, i) => i % 2 === 1)}>
              {(data) => <div>{data}</div>}
            </For>
          </div>
        </div>
      </div>
      <div>
        <div>Uint16</div>
        <For each={props.modbusData?.uint16}>{(data) => <div>{data}</div>}</For>
      </div>
      <div>
        <div>Sint16</div>
        <For each={props.modbusData?.sint16}>{(data) => <div>{data}</div>}</For>
      </div>
      <div>
        <div>Uint32</div>
        <For each={props.modbusData?.uint32}>{(data) => <div>{data}</div>}</For>
      </div>
      <div>
        <div>Sint32</div>
        <For each={props.modbusData?.sint32}>{(data) => <div>{data}</div>}</For>
      </div>
      <div>
        <div>Float32</div>
        <For each={props.modbusData?.float32}>
          {(data) => <div>{data}</div>}
        </For>
      </div>
      <div>
        <div>Uint64</div>
        <For each={props.modbusData?.uint64}>{(data) => <div>{data}</div>}</For>
      </div>
      <div>
        <div>Sint64</div>
        <For each={props.modbusData?.sint64}>{(data) => <div>{data}</div>}</For>
      </div>
      <div>
        <div>Float64</div>
        <For each={props.modbusData?.float64}>
          {(data) => <div>{data}</div>}
        </For>
      </div>
    </div>
  );
}
