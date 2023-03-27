import * as Form from "@radix-ui/react-form";
import { ContainerIcon } from "@radix-ui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { FormEvent, useState } from "react";
import { Input, Popover } from "ui";
import axios from "utils/axios";
import { isValidHttpUrl } from "utils/isValidHttpUrl";

const AddContainer = ({
  parentContainerName = "root",
}: {
  parentContainerName?: string;
}) => {
  const queryClient = useQueryClient();
  const { mutateAsync: addContainer } = useMutation({
    mutationFn: async ({ containerName }: { containerName: string }) => {
      let result = await axios.post<unknown>("/container", undefined, {
        params: {
          parent_container_name: parentContainerName,
          container_name: containerName,
        },
      });
      return result;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["bookmarks"] });
    },
  });

  const [open, setOpen] = useState(false);

  const handleSubmitAddContainer = (event: FormEvent) => {
    event.preventDefault();
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    const formJson = Object.fromEntries(formData.entries());
    const containerName = formJson["container_name"] as string;
    addContainer({ containerName }).then(() => {
      setOpen(false);
    });
  };

  return (
    <Popover
      open={open}
      onOpenChange={setOpen}
      popoverCloseProps={{ className: "hidden" }}
      popoverContentProps={{
        onClick: (event) => {
          event.stopPropagation();
        },
      }}
      popoverTrigger={
        <button
          className="group/addContainer rounded bg-transparent p-1 hover:bg-yellow-900"
          onClick={(event) => {
            event.stopPropagation();
          }}
        >
          <ContainerIcon className="group-hover/addContainer:text-yellow-300" />
        </button>
      }
    >
      <Form.Root onSubmit={handleSubmitAddContainer}>
        <Form.Field
          className="flex flex-col items-center justify-center gap-2"
          name="container_name"
        >
          <Form.Message
            className="text-sm text-fuchsia-950"
            match="valueMissing"
          >
            Please enter a container name
          </Form.Message>
          <Form.Message
            className="text-sm text-fuchsia-950"
            match={(value) => {
              return isValidHttpUrl(value);
            }}
          >
            This seems to be a url, not a container!
          </Form.Message>
          <Form.Control asChild>
            <Input placeholder="Container name" required />
          </Form.Control>
        </Form.Field>
        <Form.Submit className="hidden" />
      </Form.Root>
    </Popover>
  );
};

export default AddContainer;
