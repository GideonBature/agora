import { NextResponse } from "next/server";
import { prisma } from "@/lib/prisma";
import { withErrorHandler } from "@/lib/api-handler";

export const dynamic = "force-dynamic";

export const GET = withErrorHandler(async () => {
  const events = await prisma.event.findMany();

  const categories = Array.from(
    new Set<string>(events.map((event: { category: string }) => event.category))
  ).map((name) => ({
    name,
    icon: `/icons/${name.toLowerCase()}.svg`,
    color: "#DBF4B9",
  }));

  const popularEvents = [...events]
    .sort((a, b) => b.mintedTickets - a.mintedTickets)
    .slice(0, 8)
    .map((event) => ({
      id: event.id,
      title: event.title,
      date: event.startsAt.toLocaleString(),
      location: event.location,
      price: event.ticketPrice === 0 ? "Free" : String(event.ticketPrice),
      imageUrl: event.imageUrl,
      category: event.category,
    }));

  const organizers = Array.from(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    events.reduce((acc: any, event: any) => {
      if (!acc.has(event.organizerName)) {
        acc.set(event.organizerName, {
          id: event.organizerName.toLowerCase().replace(/\s+/g, "-"),
          title: event.organizerName,
          description: `Organizer of ${event.category} events on Agora.`,
          image: "/icons/stellar-west-africa.svg",
        });
      }
      return acc;
    }, new Map<string, { id: string; title: string; description: string; image: string }>()),
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  ).map((entry: any) => entry[1]);

  return NextResponse.json({ categories, popularEvents, organizers });
});


