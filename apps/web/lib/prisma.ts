import { PrismaClient } from "@prisma/client";

const globalForPrisma = global as unknown as { prisma: PrismaClient };

// If we are in the GitHub CI pipeline, return a dummy object so the build doesn't crash.
// Otherwise, boot up the real Prisma Client.
export const prisma =
  globalForPrisma.prisma ||
  (process.env.CI ? ({} as PrismaClient) : new PrismaClient());

if (process.env.NODE_ENV !== "production") globalForPrisma.prisma = prisma;