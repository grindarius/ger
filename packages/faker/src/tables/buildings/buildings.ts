import dayjs from 'dayjs'
import { nanoid } from 'nanoid'

import { faker } from '@faker-js/faker'

import type { Buildings } from '../../database.js'
import { coordsArrayToPoint, NANOID_LENGTH } from '../../generals.js'

export function generateBuildings (amount: number = 12): Array<Buildings> {
  return Array.from({ length: amount }, () => {
    const coords = faker.address.nearbyGPSCoordinate([16.74849678320341, 100.1916439265474], 10, true)

    const building: Buildings = {
      building_id: nanoid(NANOID_LENGTH),
      building_name: faker.commerce.productName(),
      building_coordinates: coordsArrayToPoint(coords),
      building_created_timestamp: faker.date.between(dayjs().subtract(50, 'years').toDate(), dayjs().subtract(10, 'years').toDate()).toISOString()
    }

    return building
  })
}
