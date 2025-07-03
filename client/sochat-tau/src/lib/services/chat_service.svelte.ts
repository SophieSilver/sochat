import type { Message } from "$lib/model/message";
import { defineService } from "./define_service";

/**
 * A serice representing a chat
 */
export class Chat {
  #messages: Message[] = $state([]);

  constructor({ messages = [] }: { messages?: Message[] } = {}) {
    this.#messages = messages;
  }

  get messages(): Message[] {
    return this.#messages;
  }

  async sendMessage(message: Message) {
    // TODO: actual logic
    this.#messages.push(message);
  }
}

export const [getChat, setChat] = defineService<Chat>();
