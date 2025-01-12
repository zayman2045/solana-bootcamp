"use client";

import { getCrudProgram, getCrudProgramId } from "@project/anchor";
import { useConnection } from "@solana/wallet-adapter-react";
import { Cluster, Keypair, PublicKey } from "@solana/web3.js";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useMemo } from "react";
import toast from "react-hot-toast";
import { useCluster } from "../cluster/cluster-data-access";
import { useAnchorProvider } from "../solana/solana-provider";
import { useTransactionToast } from "../ui/ui-layout";

interface CreateEntryArgs {
  title: string;
  message: string;
  owner: PublicKey;
}

export function useCrudProgram() {
  const { connection } = useConnection();
  const { cluster } = useCluster();
  const transactionToast = useTransactionToast();
  const provider = useAnchorProvider();
  const programId = useMemo(
    () => getCrudProgramId(cluster.network as Cluster),
    [cluster]
  );
  const program = useMemo(
    () => getCrudProgram(provider, programId),
    [provider, programId]
  );

  const accounts = useQuery({
    queryKey: ["entry", "all", { cluster }],
    queryFn: () => program.account.entry.all(),
  });

  const getProgramAccount = useQuery({
    queryKey: ["get-program-account", { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  });

  // A mutation that will create a new entry. The frontend will pass in the title and message.
  const createEntry = useMutation<string, Error, CreateEntryArgs>({
    mutationKey: ["entry", "create", { cluster }],
    mutationFn: async ({ title, message, owner }) => {
      return program.methods.createEntry(title, message).rpc();
    },
    onSuccess: (signature) => {
      transactionToast(signature);
      accounts.refetch();
    },
    onError: (error) => {
      toast.error(`Error creating entry: ${error.message}`);
    },
  });

  return {
    program,
    accounts,
    getProgramAccount,
    createEntry,
    programId,
  };
}

export function useCrudProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster();
  const transactionToast = useTransactionToast();
  const { program, accounts } = useCrudProgram();

  const accountQuery = useQuery({
    queryKey: ["entry", "fetch", { cluster, account }],
    queryFn: () => program.account.entry.fetch(account),
  });

  const updateEntry = useMutation<string, Error,CreateEntryArgs>({
    mutationKey: ["entry", "update", { cluster }],
    mutationFn: async ({ title, message }) => {
      return program.methods.updateEntry(title, message).rpc();
    },
    onSuccess: (signature) => {
      transactionToast(signature);
      accounts.refetch();
    },
    onError: (error) => {
      toast.error(`Error updating entry: ${error.message}`);
    },
  });

  const deleteEntry = useMutation({
    mutationKey: ["entry", "delete", { cluster }],
    mutationFn: (title: string) => {
      return program.methods.deleteEntry(title).rpc();
    },
    onSuccess: (signature) => {
      transactionToast(signature);
      accounts.refetch();
    },
    onError: (error) => {
      toast.error(`Error deleting entry: ${error.message}`);
    },
  });

  return {
    accountQuery,
    updateEntry,
    deleteEntry,
  };
}
