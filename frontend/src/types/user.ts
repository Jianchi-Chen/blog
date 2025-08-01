import z from "zod";

export const UserSchema = z.object({
  id: z.string().optional(),
  username: z.string().min(1, "不能为空"),
  password: z.string().min(1, "不能为空"),
  token: z.string().optional(),
  identity: z
    .enum(["owner", "admin", "user", "vistor"])
    .default("vistor")
    .optional(), // optional()需要放在最后
});

export type User = z.infer<typeof UserSchema>;

export const createEmptyComment = (): User => {
  return UserSchema.parse({
    username: "unknown username",
    password: "unknow password",
  });
};
