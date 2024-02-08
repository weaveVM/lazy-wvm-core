import {ContractFetcher} from "./common/contractFetcher";
import {Context} from "./context";
import {MemoryAccount} from "../index";

export type ExecutorOpts = {
    type: 'dynamic'
    fetcher: ContractFetcher,
    contractId: string
} | {
    type: 'manual',
    context: Context
};



export class Executor {
    constructor(private readonly opts: ExecutorOpts) {
    }

    async build() {
        switch (this.opts.type) {
            case "dynamic": {
                const { fetcher, contractId } = this.opts;
                const contractData = await fetcher.fetch(contractId);
                return new Context(contractId)
                    .program(contractData.code as string)
                    .programMemory(contractData.memory as MemoryAccount);
            }
            case "manual": {
                return this.opts.context;
            }
        }
    }
}