import { Resend } from "resend";

const resend = new Resend(process.env.RESEND_API_KEY);

export async function handler(event) {
  if (event.httpMethod !== "POST") {
    return {
      statusCode: 405,
      body: "Method Not Allowed",
    };
  }

  let data;
  try {
    data = JSON.parse(event.body);
  } catch {
    return {
      statusCode: 400,
      body: "Invalid JSON",
    };
  }

  const { name, email, message } = data;

  if (!name || !email || !message) {
    return {
      statusCode: 400,
      body: "Missing fields",
    };
  }

  try {
    // 1️⃣ Email YOU
    await resend.emails.send({
      from: "Contact <onboarding@resend.dev>",
      to: ["finniemcansh@gmail.com"], // <-- CHANGE THIS
      subject: "New contact form submission",
      text: `Name: ${name}\nEmail: ${email}\n\nMessage:\n${message}`,
    });

    // 2️⃣ Auto-reply to USER
    await resend.emails.send({
      from: "Contact <onboarding@resend.dev>",
      to: [email],
      subject: "Thanks for contacting me",
      text: `Hi ${name},

Thanks for reaching out!

You can contact me directly at:
YOUR_GMAIL@gmail.com

I'll get back to you as soon as I can.

— Nigel`,
    });

    return {
      statusCode: 200,
      body: JSON.stringify({ success: true }),
    };
  } catch (err) {
    console.error(err);
    return {
      statusCode: 500,
      body: "Failed to send email",
    };
  }
}

