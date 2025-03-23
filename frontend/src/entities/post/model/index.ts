import type { Tag } from "@/entities/tag/model"
import type { User } from "@/entities/user/model"

export interface Post {
  slug: string
  title: string
  body: string
  author: User
  is_published: boolean
  created_at: string
  updated_at: string | undefined
  tags: Tag[]
}
