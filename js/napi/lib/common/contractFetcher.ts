import {MemoryAccount} from "../../index";

interface ContractResult {
    contractAddress: string
    code: string | Promise<string> | Promise<Uint8Array>,
    memory: MemoryAccount | Promise<MemoryAccount>
}

export abstract class ContractFetcher {
    abstract buildUrl(contractId: string): string;

    async fetch(contractId: string): Promise<ContractResult | null> {
        const url = this.buildUrl(contractId);

        try {
            const data = await fetch(url);
            if(data.ok) {
                return data.json();
            }
        } catch(e) {
            console.error(`Error fetching contract from URL ${url}`, e);

            return null;
        }

        return null;
    }
}
