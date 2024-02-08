import {EvmContext, MemoryAccount, run} from "../";
import {EvmCall} from "./common/call";

export class Context {
    #context: EvmContext = {
        evmOpts: {
            evmConfig: {
                program: "",
                contract: {
                    address: "0x1000000000000000000000000000000000000000",
                    memory: {
                        nonce: "0",
                        balance: "0",
                        storage: {
                        },
                        code: []
                    }
                }
            }
        },
        calls: []
    };

    constructor(contractAddress?: string) {
        this.#context.evmOpts.evmConfig.contract.address = contractAddress || '0x1000000000000000000000000000000000000000';
    }

    program(compiledCode: string) {
        if(!compiledCode) {
            throw new Error("You must provide a compiled contract code when using `Context.program()`");
        }

        this.#context.evmOpts.evmConfig.program = compiledCode;
        return this;
    }

    programMemory(memory: MemoryAccount) {
        if(!memory) {
            throw new Error("You must provide a compiled memory when using `Context.programMemory()`");
        }

        this.#context.evmOpts.evmConfig.contract.memory = memory;
        return this;
    }

    programNonce(nonce: string | number) {
        if(!nonce) {
            throw new Error("You must provide a nonce when using `Context.programNonce()`");
        }

        this.#context.evmOpts.evmConfig.contract.memory.nonce = String(nonce);
        return this;
    }

    programBalance(balance: string | number) {
        if(!balance) {
            throw new Error("You must provide a balance when using `Context.programBalance()`");
        }

        this.#context.evmOpts.evmConfig.contract.memory.balance = String(balance);
        return this;
    }

    programStorage(storage: Record<string, string>) {
        if(!storage) {
            throw new Error("You must provide a storage when using `Context.storage()`");
        }

        this.#context.evmOpts.evmConfig.contract.memory.storage = storage;
        return this;
    }

    call(data: EvmCall | EvmCall[]) {
        if(!data) {
            throw new Error("You must provide a storage when using `Context.data()`");
        }

        const calls = Array.isArray(data) ? data : [data];
        calls.forEach((call) => this.#context.calls.push(call.build()));
        return this;
    }

    run() {
        return run(this.#context);
    }
}