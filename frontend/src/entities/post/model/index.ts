import type { Tag } from "@/entities/tag/model"
import type { User } from "@/entities/user/model"

export class Post {
  slug: string
  title: string
  body: string
  author: User
  is_published: boolean
  created_at: string
  updated_at: string | undefined
  tags: Tag[]

  constructor(post: Post) {
    this.slug = post.slug
    this.title = post.title
    this.body = post.body
    this.author = post.author
    this.is_published = post.is_published
    this.created_at = post.created_at
    this.updated_at = post.updated_at
    this.tags = post.tags
  }
}
