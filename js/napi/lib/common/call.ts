import {EvmCallData} from "../../index";

export class EvmCall {
    #callData?: Partial<EvmCallData> = {
        txId: undefined,
        amount: undefined,
        caller: undefined,
        input: undefined
    };

    constructor(txId: string) {
        this.#callData = {
            txId
        }
    };

    amount(amount: number | string) {
        if(!amount) {
            throw new Error("You must provide a valid amount when using `EvmCall.amount` ")
        }

        this.#callData.amount = String(amount);
        return this;
    }

    caller(caller: string) {
        if(!caller) {
            throw new Error("You must provide a valid amount when using `EvmCall.caller` ")
        }

        this.#callData.caller = caller;
        return this;
    }

    input(input: string) {
        if(!input) {
            throw new Error("You must provide a valid amount when using `EvmCall.input` ")
        }

        this.#callData.input = input;
        return this;
    }

    build(): EvmCallData {
        if(!this.#callData.txId || !this.#callData.caller || !this.#callData.input) {
            throw new Error("You must build a valid call before building `EvmCall`");
        }

        return this.#callData as EvmCallData;
    }
}