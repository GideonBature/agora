import { NextRequest, NextResponse } from "next/server";
import { prisma } from "@/lib/prisma";
import { withErrorHandler } from "@/lib/api-handler";
import { throwApiError } from "@/lib/api-errors";

type Params = {
  params: Promise<{ id: string }>;
};

export const GET = withErrorHandler(async (_request: NextRequest, { params }: Params) => {
  const { id } = await params;
  const event = await prisma.event.findUnique({
    where: { id },
  });

  if (!event) {
    throwApiError("Event not found", 404);
  }

  
  const organizerProfile = await prisma.organizerProfile
    .findUnique({ where: { address: event!.organizerWallet } })
    .catch(() => null);

  return NextResponse.json({ event, organizerProfile });
});


