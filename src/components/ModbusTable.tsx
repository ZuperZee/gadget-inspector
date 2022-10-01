import { clsxm } from "@utils/clsxm";
import { Index, JSX, splitProps } from "solid-js";

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

  ascii: string[];
}

function HeaderCell(props: JSX.ThHTMLAttributes<HTMLTableCellElement>) {
  const [p, rest] = splitProps(props, ["class"]);
  return (
    <th {...rest} class={clsxm("sticky top-0 z-20 bg-inherit p-2", p.class)}>
      {props.children}
    </th>
  );
}

function DataCell(props: JSX.TdHTMLAttributes<HTMLTableCellElement>) {
  const [p, rest] = splitProps(props, ["class"]);
  return (
    <td {...rest} class={clsxm("px-2", p.class)}>
      {props.children}
    </td>
  );
}

export function ModbusTable(props: { modbusData?: ModbusData }) {
  return (
    <div class="grow overflow-x-auto shadow-sm">
      <table class="text-left">
        <thead>
          <tr class="bg-neutral-100">
            <HeaderCell class="left-0 z-30">Addr</HeaderCell>

            <HeaderCell colSpan={2}>Binary</HeaderCell>
            <HeaderCell colSpan={2}>Ascii</HeaderCell>
            <HeaderCell colSpan={2}>Uint8</HeaderCell>
            <HeaderCell colSpan={2}>Sint8</HeaderCell>

            <HeaderCell>Uint16</HeaderCell>
            <HeaderCell>Sint16</HeaderCell>

            <HeaderCell>Uint32</HeaderCell>
            <HeaderCell>Sint32</HeaderCell>
            <HeaderCell>Float32</HeaderCell>

            <HeaderCell>Uint64</HeaderCell>
            <HeaderCell>Sint64</HeaderCell>
            <HeaderCell>Float64</HeaderCell>
          </tr>
        </thead>
        <tbody class="font-mono">
          <Index each={props.modbusData?.addresses}>
            {(_, i) => (
              <tr class="whitespace-nowrap odd:bg-white even:bg-neutral-100 [&:nth-child(4n+1)]:bg-neutral-200">
                <DataCell class="sticky left-0 z-10 bg-inherit">
                  {props.modbusData?.addresses[i]}
                </DataCell>

                <DataCell>
                  {props.modbusData?.uint8[i * 2]?.toString(2).padStart(8, "0")}
                </DataCell>
                <DataCell>
                  {props.modbusData?.uint8[i * 2 + 1]
                    ?.toString(2)
                    .padStart(8, "0")}
                </DataCell>
                <DataCell>{props.modbusData?.ascii[i * 2]}</DataCell>
                <DataCell>{props.modbusData?.ascii[i * 2 + 1]}</DataCell>
                <DataCell>{props.modbusData?.uint8[i * 2]}</DataCell>
                <DataCell>{props.modbusData?.uint8[i * 2 + 1]}</DataCell>
                <DataCell>{props.modbusData?.sint8[i * 2]}</DataCell>
                <DataCell>{props.modbusData?.sint8[i * 2 + 1]}</DataCell>

                <DataCell>{props.modbusData?.uint16[i]}</DataCell>
                <DataCell>{props.modbusData?.sint16[i]}</DataCell>

                <DataCell>{props.modbusData?.uint32[i]}</DataCell>
                <DataCell>{props.modbusData?.sint32[i]}</DataCell>
                <DataCell>{props.modbusData?.float32[i]}</DataCell>

                <DataCell>{props.modbusData?.uint64[i]}</DataCell>
                <DataCell>{props.modbusData?.sint64[i]}</DataCell>
                <DataCell class="w-full">
                  {props.modbusData?.float64[i]}
                </DataCell>
              </tr>
            )}
          </Index>
        </tbody>
      </table>
    </div>
  );
}
