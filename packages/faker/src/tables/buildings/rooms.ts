import { nanoid } from 'nanoid'
import { writeFile } from 'node:fs/promises'

import { faker } from '@faker-js/faker'

import type { Buildings, Rooms } from '../../database.js'
import { NANOID_LENGTH } from '../../generals.js'

export function generateRooms (buildings: Array<Buildings>): Array<Rooms> {
  return buildings.map(building => {
    const buildingFloorAmount = faker.datatype.number({
      min: 1,
      max: 11
    })

    const roomAmountPerFloor = faker.datatype.number({
      min: 10,
      max: 31
    })

    return Array.from({ length: buildingFloorAmount }, () => {
      let roomFloor = 1

      const roomsInTheFloor = Array.from({ length: roomAmountPerFloor }, () => {
        const room: Rooms = {
          room_id: nanoid(NANOID_LENGTH),
          building_id: building.building_id,
          room_name: faker.finance.accountName(),
          room_capacity: faker.datatype.number({
            min: 100,
            max: 301
          }),
          room_floor: roomFloor
        }

        return room
      })

      roomFloor += 1
      return roomsInTheFloor
    }).flat()
  }).flat()
}
export async function saveRooms (rooms: Array<Rooms>): Promise<void> {
  await writeFile('../../../data/rooms.json', JSON.stringify(rooms))
}
