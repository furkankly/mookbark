import * as Dialog from "@radix-ui/react-dialog";
import { Cross2Icon, MinusCircledIcon } from "@radix-ui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Button, GridZigzag } from "ui";
import axios from "utils/axios";

const DeleteEntity = ({
  entityType,
  name,
}: {
  entityType: "bookmark" | "container";
  name: string;
}) => {
  const queryClient = useQueryClient();
  const { mutate: deleteEntity } = useMutation({
    mutationFn: async ({ name }: { name: string }) => {
      let result = await axios.delete(`/${entityType}`, {
        params:
          entityType === "bookmark"
            ? { bookmark_url: name }
            : { container_name: name },
      });
      return result;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["bookmarks"] });
    },
  });

  const handleClickDeleteEntity = () => {
    deleteEntity({ name });
  };

  return (
    <>
      <Dialog.Root>
        <Dialog.Trigger asChild>
          <button
            className="group/deleteEntity rounded bg-transparent p-1 hover:bg-yellow-900"
            onClick={(e: React.MouseEvent) => e.stopPropagation()}
          >
            <MinusCircledIcon className="group-hover/deleteEntity:text-yellow-300" />
          </button>
        </Dialog.Trigger>
        <Dialog.Portal>
          <Dialog.Overlay className="animate-dialog-overlay-show fixed inset-0 z-10 bg-yellow-900/50" />
          <Dialog.Content
            onClick={(e: React.MouseEvent) => e.stopPropagation()}
            className="animate-dialog-content-show opacity-1 fixed left-1/2 top-1/2 z-20 flex w-full -translate-x-1/2 -translate-y-1/2 flex-col items-center justify-center gap-4 rounded border-2 border-yellow-300 bg-yellow-600 p-16 transition-all focus:outline-none sm:max-w-md"
          >
            <GridZigzag className="absolute -left-2 -top-2 stroke-yellow-900 stroke-2" />
            <Dialog.Title className="relative text-3xl font-bold text-yellow-300">
              Want to delete this?
            </Dialog.Title>
            <Dialog.Description className="text-center text-lg">
              In case you already didn't know ~ some actions are{" "}
              <strong className="text-green-950">irreversible</strong> in this
              life.
            </Dialog.Description>
            <div className="flex justify-end">
              <Dialog.Close asChild>
                <Button className="mt-2" onClick={handleClickDeleteEntity}>
                  Delete
                </Button>
              </Dialog.Close>
            </div>
            <Dialog.Close asChild>
              <button
                className="absolute right-2 top-2 text-yellow-300"
                aria-label="Close"
              >
                <Cross2Icon />
              </button>
            </Dialog.Close>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </>
  );
};

export default DeleteEntity;
