import z from "zod";

export const CatalogSchema = z.object({
  id: z.string().optional(),
  tag_name: z.string().optional(),
  link_article: z.array(z.string()).optional().default([]), // 关联的文章
});

export type Catalog = z.infer<typeof CatalogSchema>;

// export const createEmptyComment = (): Comment => {
//   return CommentSchema.parse({
//     content: "unknown comment",
//   });
// };
