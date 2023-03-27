import * as Form from "@radix-ui/react-form";
import { PlusIcon } from "@radix-ui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { FormEvent, useState } from "react";
import { Input, Popover } from "ui";
import axios from "utils/axios";
import { isValidHttpUrl } from "utils/isValidHttpUrl";

const AddBookmark = ({
  containerName = "root",
}: {
  containerName?: string;
}) => {
  const queryClient = useQueryClient();
  const { mutateAsync: addMookbark } = useMutation({
    mutationFn: async ({ bookmarkUrl }: { bookmarkUrl: string }) => {
      let result = await axios.post<unknown>("/bookmark", undefined, {
        params: {
          container_name: containerName,
          bookmark_url: bookmarkUrl,
        },
      });
      return result;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["bookmarks"] });
    },
  });
  const handleSubmitAddBookmark = (event: FormEvent) => {
    event.preventDefault();
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    const formJson = Object.fromEntries(formData.entries());
    const bookmarkUrl = formJson["bookmark_url"] as string;
    addMookbark({ bookmarkUrl }).then(() => {
      setOpen(false);
    });
  };

  const [open, setOpen] = useState(false);

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
          className="group/addBookmark rounded bg-transparent p-1 hover:bg-yellow-900"
          onClick={(event) => {
            event.stopPropagation();
          }}
        >
          <PlusIcon className="group-hover/addBookmark:text-yellow-300" />
        </button>
      }
    >
      <Form.Root onSubmit={handleSubmitAddBookmark}>
        <Form.Field
          className="flex flex-col items-center justify-center gap-2"
          name="bookmark_url"
        >
          <Form.Message
            className="text-sm text-fuchsia-950"
            match="valueMissing"
          >
            Please enter a url
          </Form.Message>
          <Form.Message
            className="text-sm text-fuchsia-950"
            match={(value) => {
              return !isValidHttpUrl(value);
            }}
          >
            Please provide a valid url
          </Form.Message>
          <Form.Control asChild>
            <Input placeholder="Mookbark url" required />
          </Form.Control>
        </Form.Field>
        <Form.Submit className="hidden" />
      </Form.Root>
    </Popover>
  );
};

export default AddBookmark;
