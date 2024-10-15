import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useChain } from "@cosmos-kit/react";
import assert from "assert";
import { useCallback, useEffect, useState } from "react";

// Your contract address here
const CONTRACT_ADDRESS =
  "neutron1nxshmmwrvxa2cp80nwvf03t8u5kvl2ttr8m8f43vamudsqrdvs8qqvfwpj";

const useCounter = () => {
  const { address, getCosmWasmClient, getSigningCosmWasmClient } = useChain(
    "neutronlocalnet",
    true
  );

  const [value, setValue] = useState<string | undefined>();

  const fetchValue = useCallback(async () => {
    const client = await getCosmWasmClient();

    const { current_value } = (await client.queryContractSmart(
      CONTRACT_ADDRESS,
      { current_value: {} }
    )) as { current_value: string };

    setValue(current_value);
  }, [getCosmWasmClient]);

  const increaseValue = useCallback(
    async (amount: string) => {
      assert(address, "Address is required");

      const client = await getSigningCosmWasmClient();

      const { transactionHash } = await client.execute(
        address,
        CONTRACT_ADDRESS,
        {
          increase_count: {
            amount,
          },
        },
        "auto"
      );

      void fetchValue();

      return transactionHash;
    },
    [address, getSigningCosmWasmClient, fetchValue]
  );

  useEffect(() => {
    void fetchValue();
  },[fetchValue]);

  return { value, increaseValue };
};

export const Counter = () => {
  const { address } = useChain("neutronlocalnet", true);

  const { value, increaseValue } = useCounter();

  const [amount, setAmount] = useState("");

  const handleAmountChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setAmount(event.currentTarget.value);
    },
    []
  );

  const handleIncreaseClick = useCallback(async () => {
    if (!address || !amount) return;

    const transactionHash = await increaseValue(amount);
    console.log(transactionHash);

    setAmount("");
  }, [address, amount, increaseValue]);

  return (
    <Card className="w-[350px]">
      <CardHeader>
        <CardTitle>Counter contract</CardTitle>
      </CardHeader>

      <CardContent>
        <div className="grid w-full items-center gap-4">
          <div className="flex flex-col space-y-1.5">
            <Label htmlFor="value">Current value</Label>
            <Input id="value" value={value ?? ""} disabled />
          </div>

          <div className="flex flex-col space-y-1.5">
            <Label htmlFor="amount">Amount to increase</Label>
            <Input
              id="amount"
              type="number"
              min="1"
              value={amount}
              onChange={handleAmountChange}
            />
          </div>
        </div>
      </CardContent>

      <CardFooter className="flex justify-between">
        <Button
          disabled={!address || !amount}
          type="button"
          onClick={handleIncreaseClick}
        >
          Increase
        </Button>
      </CardFooter>
    </Card>
  );
};
