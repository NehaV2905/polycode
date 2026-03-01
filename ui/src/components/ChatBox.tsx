import { useState, useRef, useEffect } from "react";

type Message = {
  id: number;
  role: "user" | "bot";
  text: string;
};

const suggestions = [
  "Find unused functions",
  "List all functions",
  "List all classes",
  "Check dependencies",
];

export default function ChatBox() {
  const [messages, setMessages] = useState<Message[]>([
    { id: 0, role: "bot", text: "Hi! Ask me anything about your code." }
  ]);
  const [input, setInput] = useState("");
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const send = (text: string) => {
    if (!text.trim()) return;
    const userMsg: Message = { id: Date.now(), role: "user", text };
    const botMsg: Message = {
      id: Date.now() + 1,
      role: "bot",
      text: "This is a placeholder response. Backend not connected yet."
    };
    setMessages(prev => [...prev, userMsg, botMsg]);
    setInput("");
  };

  return (
    <div className="chatbox">
      <h3>Ask About Code</h3>

      <div className="chat-messages">
        {messages.map(m => (
          <div key={m.id} className={`chat-bubble ${m.role}`}>
            {m.text}
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

      <div className="chat-suggestions">
        {suggestions.map(s => (
          <button key={s} className="chat-suggestion" onClick={() => send(s)}>
            {s}
          </button>
        ))}
      </div>

      <div className="chat-input-row">
        <input
          className="chat-input"
          type="text"
          placeholder="Ask about your code..."
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => e.key === "Enter" && send(input)}
        />
        <button className="chat-send" onClick={() => send(input)}>↑</button>
      </div>
    </div>
  );
}