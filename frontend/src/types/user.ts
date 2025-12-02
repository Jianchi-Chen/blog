import z from "zod";

export const UserSchema = z.object({
    id: z.string().optional(),
    username: z.string().min(1, "不能为空"),
    password: z.string().min(1, "不能为空"),
    token: z.string().optional(),
    identity: z.string().default("vistor").optional(), // optional()需要放在最后
});

export type User = z.infer<typeof UserSchema>;

export const createEmptyComment = (): User => {
    return UserSchema.parse({
        username: "unknown username",
        password: "unknow password",
    });
};

// 定义修改用户信息时的数据类型，不使用zod
export interface EditUserData {
    current_token: string;
    edited_id: string;
    edited_username: string;
    edited_password?: string;
    edited_identity: string;
}
