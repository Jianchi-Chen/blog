import z from "zod";

export const CommentSchema = z.object({
  id: z.string().optional(),
  user: z.string().optional(),
  content: z.string().min(1, "不能为空"),
  created_at: z.string().optional(),
});

export type Comment = z.infer<typeof CommentSchema>;

export const createEmptyComment = (): Comment => {
  return CommentSchema.parse({
    content: "unknown comment",
  });
};
